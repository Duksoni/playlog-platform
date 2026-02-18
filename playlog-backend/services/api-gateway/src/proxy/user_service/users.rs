use super::UserAppState;
use crate::error::Result;
use axum::{
    extract::{Path, Request, State},
    middleware::{from_fn, from_fn_with_state},
    response::Response,
    routing::{delete, get, put},
    Router,
};
use jwt_common::{auth, require_admin, require_user};
use std::sync::Arc;
use axum_macros::debug_handler;
use uuid::Uuid;

pub fn router(state: Arc<UserAppState>) -> Router<Arc<UserAppState>> {
    let jwt_config = state.jwt_config.clone();

    let public_routes = Router::new().route("/{id}", get(get_user_proxy));

    let user_routes = Router::new()
        .route("/me", put(update_user_proxy))
        .route("/me", delete(deactivate_account_proxy))
        .route("/me/change-password", put(change_password_proxy))
        .route_layer(from_fn(require_user))
        .route_layer(from_fn_with_state(jwt_config.clone(), auth));

    let admin_routes = Router::new()
        .route("/", get(find_users_proxy))
        .route("/{id}/promote", put(promote_user_proxy))
        .route("/{id}/demote", put(demote_user_proxy))
        .route("/{id}/block", put(block_user_proxy))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn_with_state(jwt_config.clone(), auth));

    Router::new()
        .merge(public_routes)
        .merge(user_routes)
        .merge(admin_routes)
}

#[debug_handler]
async fn get_user_proxy(
    State(state): State<Arc<UserAppState>>,
    Path(user_id): Path<Uuid>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();
    let path = format!("/api/users/{}", user_id);

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            &path,
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn update_user_proxy(
    State(state): State<Arc<UserAppState>>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            "/api/users/me",
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn change_password_proxy(
    State(state): State<Arc<UserAppState>>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            "/api/users/me/change-password",
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn deactivate_account_proxy(
    State(state): State<Arc<UserAppState>>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            "/api/users/me",
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn find_users_proxy(
    State(state): State<Arc<UserAppState>>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            "/users",
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn promote_user_proxy(
    State(state): State<Arc<UserAppState>>,
    Path(user_id): Path<Uuid>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();
    let path = format!("/api/users/{}/promote", user_id);

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            &path,
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn demote_user_proxy(
    State(state): State<Arc<UserAppState>>,
    Path(user_id): Path<Uuid>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();
    let path = format!("/api/users/{}/demote", user_id);

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            &path,
            parts.method,
            parts.headers,
            body,
        )
        .await
}

#[debug_handler]
async fn block_user_proxy(
    State(state): State<Arc<UserAppState>>,
    Path(user_id): Path<Uuid>,
    request: Request,
) -> Result<Response> {
    let (parts, body) = request.into_parts();
    let path = format!("/api/users/{}/block", user_id);

    state
        .proxy_client
        .forward_request(
            &state.user_service_url,
            &path,
            parts.method,
            parts.headers,
            body,
        )
        .await
}
