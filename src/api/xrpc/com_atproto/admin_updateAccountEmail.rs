use {
    crate::api::xrpc::{
        handler::{Handler, IntoHandler, Json, MethodPost},
        model::AtIdentifier,
    },
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
struct Input {
    account: AtIdentifier,
    email: String,
}

#[instrument(name = "com.atproto.admin.updateAccountEmail", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        account = %input.account,
        email = %input.email,
    );
    unimplemented!();
}

/// `com.atproto.admin.updateAccountEmail`
pub fn route() -> impl Handler {
    handler.into_handler()
}
