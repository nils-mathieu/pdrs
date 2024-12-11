use {
    super::error::XrpcError,
    crate::api::{Request, Response},
    hyper::{
        body::{Body, Bytes},
        Method,
    },
    serde::de::DeserializeOwned,
    std::{
        future::Future,
        marker::PhantomData,
        net::SocketAddr,
        ops::{Deref, DerefMut},
    },
};

/// The handler trait responsible for actually answering the request.
///
/// The whole point of this module is to turn functions into somehting that implements
/// this trait so that we can call them whenever we get a request.
pub trait Handler {
    /// The function responsible for handling the request.
    fn handle(self, req: &mut Request) -> impl Send + Future<Output = Response>;
}

/// A type can be turned into a request [`Handler`].
pub trait IntoHandler<Marker>: Send {
    /// The output handler type.
    type Handler: Handler;

    /// Turns the value into a handler.
    fn into_handler(self) -> Self::Handler;
}

/// Types that can be extracted from a request.
pub trait FromRequestParts: Sized {
    /// Extracts the type from the request.
    fn from_request_parts(parts: &Request) -> impl Send + Future<Output = Result<Self, XrpcError>>;
}

impl FromRequestParts for () {
    fn from_request_parts(
        _parts: &Request,
    ) -> impl Send + Future<Output = Result<Self, XrpcError>> {
        std::future::ready(Ok(()))
    }
}

impl FromRequestParts for SocketAddr {
    fn from_request_parts(parts: &Request) -> impl Send + Future<Output = Result<Self, XrpcError>> {
        let addr = *parts.extensions().get::<SocketAddr>().unwrap();
        std::future::ready(Ok(addr))
    }
}

/// Creates an error that indicate that the method used for the
/// provided request was not allowed.
fn method_not_allowed(req: &Request) -> XrpcError {
    XrpcError::method_not_allowed(format!(
        "Method `{}` for `{}` is not allowed",
        req.method(),
        req.uri(),
    ))
}

/// Ensures that the request method is `POST`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodPost;

impl FromRequestParts for MethodPost {
    fn from_request_parts(parts: &Request) -> impl Send + Future<Output = Result<Self, XrpcError>> {
        async move {
            if parts.method() == Method::POST {
                Ok(Self)
            } else {
                Err(method_not_allowed(parts))
            }
        }
    }
}

/// Ensures that the request method is `Get`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodGet;

impl FromRequestParts for MethodGet {
    fn from_request_parts(parts: &Request) -> impl Send + Future<Output = Result<Self, XrpcError>> {
        async move {
            if parts.method() == Method::GET {
                Ok(Self)
            } else {
                Err(method_not_allowed(parts))
            }
        }
    }
}

/// Deserializes query parameters of a request into a `T`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Query<T>(pub T);

impl<T: Send + DeserializeOwned> FromRequestParts for Query<T> {
    fn from_request_parts(parts: &Request) -> impl Send + Future<Output = Result<Self, XrpcError>> {
        let query = parts.uri().query().unwrap_or_default();
        let ret = match serde_urlencoded::from_str(query) {
            Ok(val) => Ok(Self(val)),
            Err(err) => Err(XrpcError::invalid_request(err.to_string())),
        };
        std::future::ready(ret)
    }
}

impl<T> Deref for Query<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Query<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Types that can be extracted from a request.
pub trait FromRequest: Sized {
    /// Extracts the type from the request.
    fn from_request(req: &mut Request) -> impl Send + Future<Output = Result<Self, XrpcError>>;
}

/// Types that can be turned into a response.
pub trait IntoResponse {
    /// Turns the value into a response.
    fn into_response(self) -> impl Send + Future<Output = Response>;
}

impl IntoResponse for () {
    #[inline]
    fn into_response(self) -> impl Send + Future<Output = Response> {
        std::future::ready(Response::new("".into()))
    }
}

impl<T, E> IntoResponse for Result<T, E>
where
    T: Send + IntoResponse,
    E: Send + IntoResponse,
{
    #[inline]
    fn into_response(self) -> impl Send + Future<Output = Response> {
        async move {
            match self {
                Ok(val) => val.into_response().await,
                Err(err) => err.into_response().await,
            }
        }
    }
}

/// A wrapper around a function that implements [`Handler`].
pub struct Json<T>(pub T);

impl<T> Deref for Json<T> {
    type Target = T;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> DerefMut for Json<T> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl<T> FromRequest for Json<T>
where
    T: DeserializeOwned,
{
    fn from_request(req: &mut Request) -> impl Send + Future<Output = Result<Self, XrpcError>> {
        async move {
            let body = read_body(req.body_mut()).await?;
            let val: T = serde_json::from_slice(&body)
                .map_err(|err| XrpcError::invalid_request(err.to_string()))?;
            Ok(Json(val))
        }
    }
}

/// Reads the provided body and returns it into a flat buffer.
async fn read_body<B>(body: &mut B) -> Result<Bytes, XrpcError>
where
    B: Body + Unpin,
{
    use http_body_util::BodyExt;

    match body.collect().await {
        Ok(bytes) => Ok(bytes.to_bytes()),
        Err(_err) => Err(XrpcError::DUMMY),
    }
}

/// A wrapper around a function that implements [`Handler`].
pub struct HandlerFn<F, In, Out>(pub F, PhantomData<fn(In) -> Out>);

macro_rules! impl_IntoHandler_for_fn {
    ( $($name:ident),* && $last_name:ident ) => {
        impl<Fn, Out, $($name,)* $last_name> IntoHandler<(($($name,)* $last_name,), Out)> for Fn
        where
            $($name: Send + FromRequestParts,)*
            $last_name: FromRequest,
            Fn: Send + FnOnce($($name,)* $last_name) -> Out,
            Out: Send + Future,
            Out::Output: IntoResponse,
        {
            type Handler = HandlerFn<Fn, ($($name,)* $last_name,), Out>;

            #[inline]
            fn into_handler(self) -> Self::Handler {
                HandlerFn(self, PhantomData)
            }
        }

        impl<Fn, Out, $($name,)* $last_name> Handler for HandlerFn<Fn, ($($name,)* $last_name,), Out>
        where
            $($name: Send + FromRequestParts,)*
            $last_name: FromRequest,
            Fn: Send + FnOnce($($name,)* $last_name) -> Out,
            Out: Send + Future,
            Out::Output: IntoResponse,
        {
            #[allow(non_snake_case)]
            fn handle(self, req: &mut Request) -> impl Send + Future<Output = Response> {
                async move {
                    let result: Result<_, XrpcError> = tokio::try_join!(
                        $( $name::from_request_parts(req), )*
                    );

                    let ($( $name, )*) = match result {
                        Ok(( $( $name, )* )) => ( $( $name, )* ),
                        Err(err) => return err.to_response(),
                    };

                    let __last = match $last_name::from_request(req).await {
                        Ok(val) => val,
                        Err(err) => return err.to_response(),
                    };

                    (self.0)( $( $name, )* __last, ).await.into_response().await
                }
            }
        }

        impl<Fn, Out, $($name,)*> IntoHandler<(($($name,)*), Out, ())> for Fn
        where
            $($name: Send + FromRequestParts,)*
            Fn: Send + FnOnce($($name,)*) -> Out,
            Out: Send + Future,
            Out::Output: IntoResponse,
        {
            type Handler = HandlerFn<Fn, (($($name,)*),), Out>;

            #[inline]
            fn into_handler(self) -> Self::Handler {
                HandlerFn(self, PhantomData)
            }
        }

        impl<Fn, Out, $($name,)*> Handler for HandlerFn<Fn, (($($name,)*),), Out>
        where
            $($name: Send + FromRequestParts,)*
            Fn: Send + FnOnce($($name,)*) -> Out,
            Out: Send + Future,
            Out::Output: IntoResponse,
        {
            #[allow(non_snake_case, unused_variables)]
            fn handle(self, req: &mut Request) -> impl Send + Future<Output = Response> {
                async move {
                    let result: Result<_, XrpcError> = tokio::try_join!(
                        $( $name::from_request_parts(req), )*
                    );

                    let ($( $name, )*) = match result {
                        Ok(( $( $name, )* )) => ( $( $name, )* ),
                        Err(err) => return err.to_response(),
                    };

                    (self.0)( $( $name, )*).await.into_response().await
                }
            }
        }
    };
}

impl_IntoHandler_for_fn!(&&A);
impl_IntoHandler_for_fn!(A && B);
impl_IntoHandler_for_fn!(A, B && C);
impl_IntoHandler_for_fn!(A, B, C && D);
impl_IntoHandler_for_fn!(A, B, C, D && E);
impl_IntoHandler_for_fn!(A, B, C, D, E && F);
