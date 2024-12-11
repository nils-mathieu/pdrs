use {
    super::handler::IntoResponse,
    crate::api::Response,
    hyper::{
        header::{self, HeaderValue},
        StatusCode,
    },
    serde::Serialize,
    std::{borrow::Cow, future::Future},
};

/// An XRPC error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XrpcError {
    /// The HTTP status code to return.
    pub status: StatusCode,
    /// The error code that should be used for the error.
    pub error: &'static str,
    /// A message associated with the error.
    pub message: Cow<'static, str>,
}

impl XrpcError {
    /// A dummy error.
    ///
    /// Used for example when the connection is lost.
    pub const DUMMY: Self = Self {
        status: StatusCode::INTERNAL_SERVER_ERROR,
        error: "",
        message: Cow::Borrowed(""),
    };

    /// Creates an error indcating that a resource wasn't found.
    pub fn not_found(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            status: StatusCode::NOT_FOUND,
            error: "not_found",
            message: message.into(),
        }
    }

    /// Creates an error indicating that requested method wasn't
    /// allowed.
    pub fn method_not_allowed(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            status: StatusCode::METHOD_NOT_ALLOWED,
            error: "method_not_allowed",
            message: message.into(),
        }
    }

    /// Creates an error indicating that the request was not
    /// made properly.
    pub fn invalid_request(message: impl Into<Cow<'static, str>>) -> Self {
        Self {
            status: StatusCode::BAD_REQUEST,
            error: "invalid_request",
            message: message.into(),
        }
    }

    /// Converts the error into a response.
    pub fn to_response(&self) -> Response {
        #[derive(Serialize)]
        struct Payload<'a> {
            error: &'a str,
            message: &'a str,
        }

        let payload = Payload {
            error: self.error,
            message: self.message.as_ref(),
        };
        let payload = serde_json::to_string(&payload).unwrap();

        let mut response = Response::new(payload.into());
        *response.status_mut() = self.status;
        let header = response.headers_mut();
        header.insert(header::CONTENT_TYPE, MIME_JSON);
        response
    }
}

impl IntoResponse for XrpcError {
    #[inline]
    fn into_response(self) -> impl Send + Future<Output = Response> {
        std::future::ready(self.to_response())
    }
}

/// `application/json` content type.
const MIME_JSON: HeaderValue = HeaderValue::from_static("application/json");
