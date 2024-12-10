use {
    crate::api::xrpc::{
        handler::{Handler, IntoHandler, Json, MethodPost},
        model::Did,
    },
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
struct Input {
    account: Did,
    #[serde(default)]
    note: String,
}

#[instrument(name = "com.atproto.admin.disableAccountInvites", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        account = %input.account,
        note = %input.note,
    );
    unimplemented!();
}

/// `com.atproto.admin.disableAccountInvites`
pub fn route() -> impl Handler {
    handler.into_handler()
}
