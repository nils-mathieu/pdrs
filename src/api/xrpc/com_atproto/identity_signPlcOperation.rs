use {
    crate::api::xrpc::handler::{Handler, IntoHandler, Json, MethodPost},
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    token: String,
    rotation_keys: Vec<String>,
    also_known_as: Vec<String>,
    verification_methods: (),
    services: (),
}

#[instrument(name = "com.atproto.identity.signPlcOperation", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        token = %input.token,
        rotation_keys = ?input.rotation_keys,
        also_known_as = ?input.also_known_as,
        verification_methods = ?input.verification_methods,
        services = ?input.services,
    );
    unimplemented!();
}

/// `com.atproto.identity.signPlcOperation`
pub fn route() -> impl Handler {
    handler.into_handler()
}
