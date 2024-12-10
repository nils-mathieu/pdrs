//! Defines the XRPC routes that the server is able to respond to.

mod error;
mod handler;
mod model;

mod com_atproto;

use {
    self::{error::XrpcError, handler::Handler},
    super::{split_uri_path, Request, Response},
};

/// Handles an XRPC request and returns an appropriate response.
pub async fn handle_request(rest: &[u8], req: &mut Request) -> Response {
    let (mut nsid, _) = split_uri_path(rest);

    nsid = nsid.get(1..).unwrap_or_default();

    match nsid {
        b"com.atproto.admin.deleteAccount" => {
            self::com_atproto::admin_deleteAccount::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.disableAccountInvites" => {
            self::com_atproto::admin_disableAccountInvites::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.disableInviteCodes" => {
            self::com_atproto::admin_disableInviteCodes::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.enableAccountInvites" => {
            self::com_atproto::admin_enableAccountInvites::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.getAccountInfo" => {
            self::com_atproto::admin_getAccountInfo::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.getAccountInfos" => {
            self::com_atproto::admin_getAccountInfos::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.getInviteCodes" => {
            self::com_atproto::admin_getInviteCodes::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.getSubjectStatus" => {
            self::com_atproto::admin_getSubjectStatus::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.searchAccounts" => {
            self::com_atproto::admin_searchAccounts::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.sendEmail" => {
            self::com_atproto::admin_sendEmail::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.updateAccountEmail" => {
            self::com_atproto::admin_updateAccountEmail::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.updateAccountHandle" => {
            self::com_atproto::admin_updateAccountHandle::route()
                .handle(req)
                .await
        }
        b"com.atproto.admin.updateAccountPassword" => {
            self::com_atproto::admin_updateAccountPassword::route()
                .handle(req)
                .await
        }
        b"com.proto.admin.updateSubjectStatus" => {
            self::com_atproto::admin_updateSubjectStatus::route()
                .handle(req)
                .await
        }
        b"com.proto.identity.getRecommendedDidCredentials" => {
            self::com_atproto::identity_getRecommendedDidCredentials::route()
                .handle(req)
                .await
        }
        b"com.proto.identity.requestPlcOperationSignature" => {
            self::com_atproto::identity_requestPlcOperationSignature::route()
                .handle(req)
                .await
        }
        b"com.proto.identity.resolveHandle" => {
            self::com_atproto::identity_resolveHandle::route()
                .handle(req)
                .await
        }
        b"com.proto.identity.signPlcOperation" => {
            self::com_atproto::identity_signPlcOperation::route()
                .handle(req)
                .await
        }
        b"com.proto.identity.submitPlcOperation" => {
            self::com_atproto::identity_submitPlcOperation::route()
                .handle(req)
                .await
        }
        b"com.proto.identity.updateHandle" => {
            self::com_atproto::identity_updateHandle::route()
                .handle(req)
                .await
        }
        _ => XrpcError::NotFound.to_response(),
    }
}
