use {
    crate::api::xrpc::handler::{Handler, IntoHandler, MethodGet, Query},
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
struct Input {
    did: String,
}

#[instrument(name = "com.atproto.admin.getAccountInfo", skip_all)]
async fn handler(_: MethodGet, input: Query<Input>) {
    info!(did = %input.did);
    unimplemented!();
}

/// `com.atproto.admin.getAccountInfo`
pub fn route() -> impl Handler {
    handler.into_handler()
}
