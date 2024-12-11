//! Defines the XRPC routes that the server is able to respond to.

mod error;
mod handler;
mod model;

mod com_atproto;

use {
    self::{
        error::XrpcError,
        handler::{Handler, IntoHandler},
    },
    super::{split_uri_path, Request, Response},
};

/// Handles an XRPC request and returns an appropriate response.
pub async fn handle_request(rest: &[u8], req: &mut Request) -> Response {
    let (mut nsid, _) = split_uri_path(rest);

    nsid = nsid.get(1..).unwrap_or_default();

    match nsid {
        b"com.atproto.admin.deleteAccount" => {
            self::com_atproto::admin_deleteAccount::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.disableAccountInvites" => {
            self::com_atproto::admin_disableAccountInvites::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.disableInviteCodes" => {
            self::com_atproto::admin_disableInviteCodes::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.enableAccountInvites" => {
            self::com_atproto::admin_enableAccountInvites::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.getAccountInfo" => {
            self::com_atproto::admin_getAccountInfo::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.getAccountInfos" => {
            self::com_atproto::admin_getAccountInfos::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.getInviteCodes" => {
            self::com_atproto::admin_getInviteCodes::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.getSubjectStatus" => {
            self::com_atproto::admin_getSubjectStatus::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.searchAccounts" => {
            self::com_atproto::admin_searchAccounts::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.sendEmail" => {
            self::com_atproto::admin_sendEmail::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.updateAccountEmail" => {
            self::com_atproto::admin_updateAccountEmail::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.updateAccountHandle" => {
            self::com_atproto::admin_updateAccountHandle::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.updateAccountPassword" => {
            self::com_atproto::admin_updateAccountPassword::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.admin.updateSubjectStatus" => {
            self::com_atproto::admin_updateSubjectStatus::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.identity.getRecommendedDidCredentials" => {
            self::com_atproto::identity_getRecommendedDidCredentials::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.identity.requestPlcOperationSignature" => {
            self::com_atproto::identity_requestPlcOperationSignature::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.identity.resolveHandle" => {
            self::com_atproto::identity_resolveHandle::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.identity.signPlcOperation" => {
            self::com_atproto::identity_signPlcOperation::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.identity.submitPlcOperation" => {
            self::com_atproto::identity_submitPlcOperation::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.identity.updateHandle" => {
            self::com_atproto::identity_updateHandle::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.label.queryLabels" => {
            self::com_atproto::label_queryLabels::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.repo.applyWrites" => {
            self::com_atproto::repo_applyWrites::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.repo.createRecord" => {
            self::com_atproto::repo_createRecord::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.repo.describeRepo" => {
            self::com_atproto::repo_describeRepo::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.repo.getRecord" => {
            self::com_atproto::repo_getRecord::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.repo.importRepo" => {
            self::com_atproto::repo_importRepo::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.repo.listMissingBlobs" => {
            self::com_atproto::repo_listMissingBlobs::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.repo.listRecords" => {
            self::com_atproto::repo_listRecords::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.repo.putRecord" => {
            self::com_atproto::repo_putRecord::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.repo.uploadBlob" => {
            self::com_atproto::repo_uploadBlob::handler
                .into_handler()
                .handle(req)
                .await
        }
        _ => XrpcError::NotFound.to_response(),
    }
}
