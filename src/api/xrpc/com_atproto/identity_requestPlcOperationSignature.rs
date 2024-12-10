use {
    crate::api::xrpc::handler::{Handler, IntoHandler, MethodPost},
    tracing::instrument,
};

#[instrument(name = "com.atproto.identity.requestPlcOperationSignature", skip_all)]
async fn handler(_: MethodPost) {
    unimplemented!();
}

/// `com.atproto.identity.getPrequestPlcOperationSignature`
pub fn route() -> impl Handler {
    handler.into_handler()
}
