use {
    super::handler::IntoResponse,
    crate::api::Response,
    hyper::{
        header::{self, HeaderValue},
        StatusCode,
    },
    serde::Serialize,
    std::future::Future,
};

/// An XRPC error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XrpcError {
    NotFound,
    MethodNotAllowed,
    InvalidRequest,
    ConnectionError,
}

impl XrpcError {
    /// Returns the status code that should be used for the error.
    pub fn status_code(self) -> StatusCode {
        match self {
            XrpcError::NotFound => StatusCode::NOT_FOUND,
            XrpcError::InvalidRequest => StatusCode::BAD_REQUEST,
            XrpcError::ConnectionError => StatusCode::INTERNAL_SERVER_ERROR,
            XrpcError::MethodNotAllowed => StatusCode::METHOD_NOT_ALLOWED,
        }
    }

    /// Returns the description of the error.
    pub fn message(self) -> &'static str {
        match self {
            XrpcError::NotFound => "The requested resource was not found.",
            XrpcError::InvalidRequest => "The input provided was invalid.",
            XrpcError::ConnectionError => "An error occurred while communicating.",
            XrpcError::MethodNotAllowed => "The method is not allowed.",
        }
    }

    /// Returns the error code that should be used for the error.
    pub fn code(self) -> &'static str {
        match self {
            XrpcError::NotFound => "NotFound",
            XrpcError::InvalidRequest => "InvalidRequest",
            XrpcError::ConnectionError => "ConnectionError",
            XrpcError::MethodNotAllowed => "MethodNotAllowed",
        }
    }

    /// Converts the error into a response.
    pub fn to_response(self) -> Response {
        #[derive(Serialize)]
        struct Payload {
            error: &'static str,
            message: &'static str,
        }

        let payload = Payload {
            error: self.code(),
            message: self.message(),
        };
        let payload = serde_json::to_string(&payload).unwrap();

        let mut response = Response::new(payload.into());
        *response.status_mut() = self.status_code();
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
