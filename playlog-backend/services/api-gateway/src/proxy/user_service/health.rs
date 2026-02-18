use super::UserAppState;
use crate::error::Result;
use axum::{
    extract::{Request, State},
    response::Response,
    routing::get,
    Router
};
use std::sync::Arc;

pub fn router() -> Router<Arc<UserAppState>> {
    Router::new().route("/user-service-health", get(user_service_health_proxy))
}

/// Proxy health check requests to user-service
async fn user_service_health_proxy(
    State(state): State<Arc<UserAppState>>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            "/api/user-service-health",
            parts.method,
            parts.headers,
            body,
        )
        .await
}
