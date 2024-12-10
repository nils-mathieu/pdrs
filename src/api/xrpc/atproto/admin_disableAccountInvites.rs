use {
    crate::api::xrpc::handler::{Handler, IntoHandler, Json, MethodPost},
    tracing::info,
};

#[derive(serde::Deserialize)]
struct Input {
    account: String,
    note: Option<String>,
}

#[tracing::instrument(name = "com.atproto.admin.disableAccountInvites", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        account = %input.account,
        note = %input.note.as_deref().unwrap_or_default(),
    );
    unimplemented!();
}

/// `com.atproto.admin.disableAccountInvites`
pub fn route() -> impl Handler {
    handler.into_handler()
}
