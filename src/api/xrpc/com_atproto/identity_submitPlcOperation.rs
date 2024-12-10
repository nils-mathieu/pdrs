use {
    crate::api::xrpc::handler::{Handler, IntoHandler, Json, MethodPost},
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    operation: (),
}

#[instrument(name = "com.atproto.identity.submitPlcOperation", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        operation = ?input.operation,
    );
    unimplemented!();
}

/// `com.atproto.identity.submitPlcOperation`
pub fn route() -> impl Handler {
    handler.into_handler()
}
