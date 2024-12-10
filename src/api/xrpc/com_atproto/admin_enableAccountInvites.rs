use {
    crate::api::xrpc::handler::{Handler, IntoHandler, Json, MethodPost},
    serde::Deserialize,
    tracing::{info, instrument},
};

#[derive(Deserialize)]
struct Input {
    account: String,
    #[serde(default)]
    note: String,
}

#[instrument(name = "com.atproto.admin.enableAccountInvites", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        account = %input.account,
        note = %input.note,
    );
    unimplemented!();
}

/// `com.atproto.admin.enableAccountInvites`
pub fn route() -> impl Handler {
    handler.into_handler()
}
