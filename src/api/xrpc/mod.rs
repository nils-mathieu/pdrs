//! Defines the XRPC routes that the server is able to respond to.

mod error;
pub use self::error::*;

use super::{split_uri_path, Request, Response};

/// Handles an XRPC request and returns an appropriate response.
pub fn handle_request(rest: &[u8], _req: &Request) -> Response {
    let (_nsid, _) = split_uri_path(rest);
    XrpcError::NotFound.to_response()
}
