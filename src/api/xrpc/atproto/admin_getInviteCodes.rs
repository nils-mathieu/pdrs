use {
    crate::api::xrpc::{
        error::XrpcError,
        handler::{Handler, IntoHandler, MethodGet, Query},
    },
    serde::Deserialize,
    tracing::{info, instrument},
};

#[derive(Default, Deserialize)]
#[serde(rename_all = "snake_case")]
enum Sort {
    #[default]
    Recent,
    Usage,
}

#[derive(Deserialize)]
struct Input {
    #[serde(default)]
    sort: String,
    #[serde(default = "default_limit")]
    limit: u32,
    #[serde(default)]
    cursor: Option<String>,
}

fn default_limit() -> u32 {
    100
}

#[instrument(name = "com.atproto.admin.getInviteCodes", skip_all)]
async fn handler(_: MethodGet, input: Query<Input>) -> Result<(), XrpcError> {
    if !(1..=500).contains(&input.limit) {
        return Err(XrpcError::InvalidRequest);
    }

    info!(
        sort = ?input.sort,
        limit = input.limit,
        cursor = ?input.cursor,
    );
    unimplemented!();
}

/// com.atproto.admin.getInviteCodes
pub fn route() -> impl Handler {
    handler.into_handler()
}
