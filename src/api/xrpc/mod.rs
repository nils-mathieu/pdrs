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
        b"/com.atproto.admin.enableAccountInvites" => {
            self::atproto::admin_enableAccountInvites::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.getAccountInfo" => {
            self::atproto::admin_getAccountInfo::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.getAccountInfos" => {
            self::atproto::admin_getAccountInfos::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.getInviteCodes" => {
            self::atproto::admin_getInviteCodes::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.getSubjectStatus" => {
            self::atproto::admin_getSubjectStatus::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.searchAccounts" => {
            self::atproto::admin_searchAccounts::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.sendEmail" => {
            self::atproto::admin_sendEmail::route().handle(req).await
        }
        b"/com.atproto.admin.updateAccountEmail" => {
            self::atproto::admin_updateAccountEmail::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.updateAccountHandle" => {
            self::atproto::admin_updateAccountHandle::route()
                .handle(req)
                .await
        }
        b"/com.atproto.admin.updateAccountPassword" => {
            self::atproto::admin_updateAccountPassword::route()
                .handle(req)
                .await
        }
        b"/com.proto.admin.updateSubjectStatus" => {
            self::atproto::admin_updateSubjectStatus::route()
                .handle(req)
                .await
        }
        _ => XrpcError::NotFound.to_response(),
    }
}
