use {
    crate::api::xrpc::{
        handler::{Handler, IntoHandler, Json, MethodPost},
        model::Did,
    },
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
struct Input {
    did: Did,
    password: String,
}

#[instrument(name = "com.atproto.admin.updateAccountPassword", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        did = %input.did,
        password = %input.password,
    );
    unimplemented!();
}

/// `com.atproto.admin.updateAccountHandle`
pub fn route() -> impl Handler {
    handler.into_handler()
}
