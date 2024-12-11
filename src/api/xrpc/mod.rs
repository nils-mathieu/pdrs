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
        b"com.atproto.repo.deleteRecord" => {
            self::com_atproto::repo_deleteRecord::handler
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
        b"com.atproto.server.activateAccount" => {
            self::com_atproto::server_activateAccount::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.checkAccountStatus" => {
            self::com_atproto::server_checkAccountStatus::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.confirmEmail" => {
            self::com_atproto::server_confirmEmail::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.createAccount" => {
            self::com_atproto::server_createAccount::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.createAppPassword" => {
            self::com_atproto::server_createAppPassword::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.createInviteCode" => {
            self::com_atproto::server_createInviteCode::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.createInviteCodes" => {
            self::com_atproto::server_createInviteCodes::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.createSession" => {
            self::com_atproto::server_createSession::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.deactivateAccount" => {
            self::com_atproto::server_deactivateAccount::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.deleteAccount" => {
            self::com_atproto::server_deleteAccount::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.deleteSession" => {
            self::com_atproto::server_deleteSession::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.describeServer" => {
            self::com_atproto::server_describeServer::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.getAccountInviteCodes" => {
            self::com_atproto::server_getAccountInviteCodes::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.getServiceAuth" => {
            self::com_atproto::server_getServiceAuth::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.getSession" => {
            self::com_atproto::server_getSession::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.listAppPasswords" => {
            self::com_atproto::server_listAppPasswords::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.refreshSession" => {
            self::com_atproto::server_refreshSession::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.requestAccountDelete" => {
            self::com_atproto::server_requestAccountDelete::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.requestEmailConfirmation" => {
            self::com_atproto::server_requestEmailConfirmation::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.requestEmailUpdate" => {
            self::com_atproto::server_requestEmailUpdate::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.requestPasswordReset" => {
            self::com_atproto::server_requestPasswordReset::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.reserveSigningKey" => {
            self::com_atproto::server_reserveSigningKey::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.resetPassword" => {
            self::com_atproto::server_resetPassword::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.revokeAppPassword" => {
            self::com_atproto::server_revokeAppPassword::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.server.updateEmail" => {
            self::com_atproto::server_updateEmail::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.getBlob" => {
            self::com_atproto::sync_getBlob::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.getBlocks" => {
            self::com_atproto::sync_getBlocks::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.getLatestCommit" => {
            self::com_atproto::sync_getLatestCommit::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.getRecord" => {
            self::com_atproto::sync_getRecord::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.getRepo" => {
            self::com_atproto::sync_getRepo::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.getRepoStatus" => {
            self::com_atproto::sync_getRepoStatus::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.listBlobs" => {
            self::com_atproto::sync_listBlobs::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.listRepos" => {
            self::com_atproto::sync_listRepos::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.notifyOfUpdate" => {
            self::com_atproto::sync_notifyOfUpdate::handler
                .into_handler()
                .handle(req)
                .await
        }
        b"com.atproto.sync.requestCrawl" => {
            self::com_atproto::sync_requestCrawl::handler
                .into_handler()
                .handle(req)
                .await
        }
        _ => XrpcError::NotFound.to_response(),
    }
}
