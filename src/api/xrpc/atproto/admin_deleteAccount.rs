use {
    crate::api::xrpc::handler::{Handler, IntoHandler, Json, MethodPost},
    tracing::info,
};

#[derive(serde::Deserialize)]
struct Input {
    did: String,
}

#[tracing::instrument(name = "com.atproto.admin.deleteAccount", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(did = %input.did);
    unimplemented!();
}

/// `com.atproto.admin.deleteAccount`
pub fn route() -> impl Handler {
    handler.into_handler()
}
