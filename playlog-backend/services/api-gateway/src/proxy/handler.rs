use super::ServiceAppState;
use crate::error::Result;
use axum::{
    extract::{OriginalUri, Request, State},
    response::Response,
};
use axum_macros::debug_handler;
use std::sync::Arc;

#[debug_handler]
pub async fn proxy_handler(
    State(state): State<Arc<ServiceAppState>>,
    OriginalUri(original_uri): OriginalUri,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();

    let path = original_uri.path();
    let path = if let Some(query) = parts.uri.query() {
        format!("{}?{}", path, query)
    } else {
        path.to_string()
    };

    state
        .proxy_client
        .forward_request(&state.service_url, &path, parts.method, parts.headers, body)
        .await
}
