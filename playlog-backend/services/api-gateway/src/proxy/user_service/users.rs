use crate::proxy::{proxy_handler, ServiceAppState};
use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, put},
    Router,
};
use jwt_common::{auth, require_admin, require_user};
use std::sync::Arc;

pub fn router(state: Arc<ServiceAppState>) -> Router<Arc<ServiceAppState>> {
    let jwt_config = state.jwt_config.clone();

    let public_routes = Router::new().route("/{id}", get(proxy_handler));

    let user_routes = Router::new()
        .route("/me", put(proxy_handler))
        .route("/me", delete(proxy_handler))
        .route("/me/change-password", put(proxy_handler))
        .route_layer(from_fn(require_user))
        .route_layer(from_fn_with_state(jwt_config.clone(), auth));

    let admin_routes = Router::new()
        .route("/", get(proxy_handler))
        .route("/{id}/promote", put(proxy_handler))
        .route("/{id}/demote", put(proxy_handler))
        .route("/{id}/block", put(proxy_handler))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn_with_state(jwt_config.clone(), auth));

    Router::new()
        .merge(public_routes)
        .merge(user_routes)
        .merge(admin_routes)
}
