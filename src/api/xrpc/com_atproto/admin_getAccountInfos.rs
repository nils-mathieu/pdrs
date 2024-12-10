use {
    crate::api::xrpc::handler::{Handler, IntoHandler, MethodGet, Query},
    serde::Deserialize,
    tracing::{info, instrument},
};

#[derive(Deserialize)]
struct Input {
    dids: Vec<String>,
}

#[instrument(name = "com.atproto.admin.getAccountInfos", skip_all)]
async fn handler(_: MethodGet, input: Query<Input>) {
    info!(dids = %input.dids.join(" "));
    unimplemented!();
}

/// `com.atproto.admin.getAccountInfos`
pub fn route() -> impl Handler {
    handler.into_handler()
}
