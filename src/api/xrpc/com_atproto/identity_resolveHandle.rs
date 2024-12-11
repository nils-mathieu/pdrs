use {
    crate::api::xrpc::{
        handler::{Handler, IntoHandler, MethodGet, Query},
        model::Handle,
    },
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
struct Input {
    handle: Handle,
}

#[instrument(name = "com.atproto.identity.resolveHandle", skip_all)]
async fn handler(_: MethodGet, input: Query<Input>) {
    info!(
        handle = ?input.handle,
    );
    unimplemented!();
}

/// `com.atproto.identity.resolveHandle`
pub fn route() -> impl Handler {
    handler.into_handler()
}
