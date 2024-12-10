use {
    crate::api::xrpc::handler::{Handler, IntoHandler, MethodPost},
    tracing::instrument,
};

#[instrument(name = "com.atproto.admin.updateSubjectStatus", skip_all)]
async fn handler(_: MethodPost) {
    unimplemented!();
}

/// `com.atproto.admin.updateSubjectStatus`
pub fn route() -> impl Handler {
    handler.into_handler()
}
