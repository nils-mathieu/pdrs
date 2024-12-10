use {
    crate::api::xrpc::{
        handler::{Handler, IntoHandler, Json, MethodPost},
        model::Did,
    },
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
#[serde(rename_all = "camelCase")]
struct Input {
    recipient_did: Did,
    content: String,
    #[serde(default)]
    subject: Option<String>,
    sender_did: Did,
    #[serde(default)]
    comment: String,
}

#[instrument(name = "com.atproto.admin.sendEmail", skip_all)]
async fn handler(_: MethodPost, input: Json<Input>) {
    info!(
        recipient_did = %input.recipient_did,
        content = %input.content,
        subject = input.subject.as_deref().unwrap_or_default(),
        sender_did = %input.sender_did,
        comment = %input.comment,
    );
    unimplemented!();
}

/// `com.atproto.admin.sendEmail`
pub fn route() -> impl Handler {
    handler.into_handler()
}
