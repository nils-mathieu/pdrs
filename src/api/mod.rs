//! Defines the routes that the server may respond to.

use {
    crate::panic::trace_payload,
    futures::FutureExt,
    hyper::{
        body::Bytes,
        header::{self, HeaderValue},
        StatusCode,
    },
    std::{net::SocketAddr, panic::AssertUnwindSafe},
};

mod xrpc;

/// The input request type used by the [`handle_request`] function.
pub type Request = hyper::Request<hyper::body::Incoming>;

/// The output response type used by the [`handle_request`] function.
pub type Response = hyper::Response<http_body_util::Full<Bytes>>;

/// Handles a request and returns an appropriate response.
pub async fn handle_request(addr: &SocketAddr, request: &Request) -> Response {
    let fut = handle_request_inner(addr, request);

    match AssertUnwindSafe(fut).catch_unwind().await {
        Ok(response) => response,
        Err(payload) => {
            trace_payload(&payload, None);

            let mut response = Response::new("".into());
            *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
            response
        }
    }
}

/// Handles a request and returns an appropriate response.
///
/// Unlike [`handle_request`], this function always returns a [`Response`] sucessfully. If an unexpected
/// error occurs, the function will panic. Note that panics here should be considered bugs or unlikely
/// edge cases (such as memory running out).
async fn handle_request_inner(_addr: &SocketAddr, request: &Request) -> Response {
    let (uri_first_part, uri_rest) = split_uri_path(request.uri().path().as_bytes());

    match uri_first_part {
        b"" | b"/" => file(include_bytes!("index.html"), MIME_HTML),
        b"/robots.txt" => file(include_bytes!("robots.txt"), MIME_TEXT),
        b"/xrpc" => self::xrpc::handle_request(uri_rest, request),
        _ => not_found(),
    }
}

/// Creates a [`Response`] that indicates that the requested resource was not
/// found.
///
/// Not additional information is provided, and the body of the response is left
/// empty.
fn not_found() -> Response {
    let mut response = Response::new("".into());
    *response.status_mut() = StatusCode::NOT_FOUND;
    response
}

/// `text/plain; charset=utf-8` content type.
const MIME_TEXT: HeaderValue = HeaderValue::from_static("text/plain; charset=utf-8");
/// `text/html` content type.
const MIME_HTML: HeaderValue = HeaderValue::from_static("text/html");

/// Creates a [`Response`] that contains `data` with the provided content type.
fn file(data: &'static [u8], content_type: HeaderValue) -> Response {
    let mut response = Response::new(data.into());
    *response.status_mut() = StatusCode::OK;
    let headers = response.headers_mut();
    headers.insert(header::CONTENT_TYPE, content_type);
    response
}

/// Splits the provided URI path into two parts:
///
/// 1. The first part of the path. This may be empty if the path is empty.
///
/// 2. The remaining part of the path. This might be empty if the path only
///    contains a single part (or no parts).
fn split_uri_path(path: &[u8]) -> (&[u8], &[u8]) {
    if path.is_empty() {
        return (b"", b"");
    }

    let skipped_first = unsafe { path.get_unchecked(1..) };

    match memchr::memchr(b'/', skipped_first) {
        Some(index) => {
            let split_index = index + 1;
            let first = unsafe { path.get_unchecked(..split_index) };
            let rest = unsafe { path.get_unchecked(split_index..) };
            (first, rest)
        }
        None => (path, b""),
    }
}
