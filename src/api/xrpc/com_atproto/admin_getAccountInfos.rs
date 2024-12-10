use {
    crate::api::xrpc::{
        handler::{Handler, IntoHandler, MethodGet, Query},
        model::Did,
    },
    serde::Deserialize,
    tracing::{info, instrument},
};

#[derive(Deserialize)]
struct Input {
    dids: Vec<Did>,
}

#[instrument(name = "com.atproto.admin.getAccountInfos", skip_all)]
async fn handler(_: MethodGet, input: Query<Input>) {
    info!(dids = ?input.dids);
    unimplemented!();
}

/// `com.atproto.admin.getAccountInfos`
pub fn route() -> impl Handler {
    handler.into_handler()
}
