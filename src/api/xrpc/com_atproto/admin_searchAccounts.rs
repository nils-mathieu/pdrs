use {
    crate::api::xrpc::{
        error::XrpcError,
        handler::{Handler, IntoHandler, MethodGet, Query},
    },
    tracing::{info, instrument},
};

#[derive(serde::Deserialize)]
struct Input {
    #[serde(default)]
    email: Option<String>,
    #[serde(default)]
    cursor: Option<String>,
    #[serde(default = "default_limit")]
    limit: u32,
}

fn default_limit() -> u32 {
    50
}

#[instrument(name = "com.atproto.admin.getSubjectStatus", skip_all)]
async fn handler(_: MethodGet, input: Query<Input>) -> Result<(), XrpcError> {
    if !(1..=100).contains(&input.limit) {
        return Err(XrpcError::InvalidRequest);
    }

    info!(
        email = %input.email.as_deref().unwrap_or_default(),
        cursor = input.cursor.as_deref().unwrap_or_default(),
        limit = input.limit,
    );
    unimplemented!();
}

/// `com.atproto.admin.getSubjectStatus`
pub fn route() -> impl Handler {
    handler.into_handler()
}
