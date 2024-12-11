use {
    crate::api::xrpc::{
        handler::{Handler, IntoHandler, Json, MethodPost},
        model::{Did, Handle},
    },
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
struct Input {
    did: Did,
    handle: Handle,
}

#[instrument(name = "com.atproto.admin.updateAccountHandle", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        did = %input.did,
        handle = %input.handle,
    );
    unimplemented!();
}

/// `com.atproto.admin.updateAccountHandle`
pub fn route() -> impl Handler {
    handler.into_handler()
}
