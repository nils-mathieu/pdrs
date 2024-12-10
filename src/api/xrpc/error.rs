use {
    crate::api::Response,
    hyper::{
        header::{self, HeaderValue},
        StatusCode,
    },
    serde::Serialize,
};

/// An XRPC error.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XrpcError {
    NotFound,
}

impl XrpcError {
    /// Returns the status code that should be used for the error.
    pub fn status_code(self) -> StatusCode {
        match self {
            XrpcError::NotFound => StatusCode::NOT_FOUND,
        }
    }

    /// Returns the description of the error.
    pub fn message(self) -> &'static str {
        match self {
            XrpcError::NotFound => "The requested resource was not found.",
        }
    }

    /// Returns the error code that should be used for the error.
    pub fn code(self) -> &'static str {
        match self {
            XrpcError::NotFound => "not_found",
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

/// `application/json` content type.
const MIME_JSON: HeaderValue = HeaderValue::from_static("application/json");
