use {
    crate::api::xrpc::handler::{Handler, IntoHandler, Json, MethodPost},
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
struct Input {
    codes: Vec<String>,
    accounts: Vec<String>,
}

#[instrument(name = "com.atproto.admin.disableInviteCodes", skip_all)]
pub async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        codes = ?input.codes,
        accounts = ?input.accounts,
    );
    unimplemented!();
}

/// `com.atproto.admin.disableInviteCodes`
pub fn route() -> impl Handler {
    handler.into_handler()
}
