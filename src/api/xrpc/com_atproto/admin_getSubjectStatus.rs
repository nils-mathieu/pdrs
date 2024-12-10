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
    #[serde(default)]
    did: Option<Did>,
    #[serde(default)]
    uri: Option<String>,
    #[serde(default)]
    blob: Option<String>,
}

#[instrument(name = "com.atproto.admin.getAccountInfo", skip_all)]
async fn handler(_: MethodGet, input: Query<Input>) {
    info!(
        did = ?input.did,
        uri = %input.uri.as_deref().unwrap_or_default(),
        blob = %input.blob.as_deref().unwrap_or_default(),
    );
    unimplemented!();
}

/// `com.atproto.admin.getAccountInfo`
pub fn route() -> impl Handler {
    handler.into_handler()
}
