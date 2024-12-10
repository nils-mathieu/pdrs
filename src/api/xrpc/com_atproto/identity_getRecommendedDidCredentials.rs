use {
    crate::api::xrpc::handler::{Handler, IntoHandler, MethodGet},
    tracing::instrument,
};

#[instrument(name = "com.atproto.identity.getRecommendedDidCredentials", skip_all)]
async fn handler(_: MethodGet) {
    unimplemented!();
}

/// `com.atproto.identity.getRecommendedDidCredentials`
pub fn route() -> impl Handler {
    handler.into_handler()
}
