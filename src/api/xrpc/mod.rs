//! Defines the XRPC routes that the server is able to respond to.

mod error;
mod handler;

mod atproto;

use {
    self::{error::XrpcError, handler::Handler},
    super::{split_uri_path, Request, Response},
};

/// Handles an XRPC request and returns an appropriate response.
pub async fn handle_request(rest: &[u8], req: &mut Request) -> Response {
    let (nsid, _) = split_uri_path(rest);

    match nsid {
        b"/com.atproto.admin.deleteAccount" => {
            self::atproto::admin_deleteAccount::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.disableAccountInvites" => {
            self::atproto::admin_disableAccountInvites::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.disableInviteCodes" => {
            self::atproto::admin_disableInviteCodes::route()
                .handle(req)
                .await
        }
        _ => XrpcError::NotFound.to_response(),
    }
}
