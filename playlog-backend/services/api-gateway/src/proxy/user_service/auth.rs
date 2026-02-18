use super::UserAppState;
use crate::error::Result;
use axum::{
    extract::{Request, State},
    response::Response,
    routing::post,
    Router,
};
use axum_macros::debug_handler;
use std::sync::Arc;

pub fn router() -> Router<Arc<UserAppState>> {
    Router::new()
        .route("/login", post(login_proxy))
        .route("/register", post(register_proxy))
        .route("/logout", post(logout_proxy))
        .route("/refresh", post(refresh_tokens_proxy))
}

#[debug_handler]
async fn login_proxy(State(state): State<Arc<UserAppState>>, request: Request) -> Result<Response> {
    let (parts, body) = request.into_parts();

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            "/api/auth/login",
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn register_proxy(
    State(state): State<Arc<UserAppState>>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            "/api/auth/register",
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn logout_proxy(
    State(state): State<Arc<UserAppState>>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            "/api/auth/logout",
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn refresh_tokens_proxy(
    State(state): State<Arc<UserAppState>>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            "/api/auth/refresh",
            parts.method,
            parts.headers,
            body,
        )
        .await
}
