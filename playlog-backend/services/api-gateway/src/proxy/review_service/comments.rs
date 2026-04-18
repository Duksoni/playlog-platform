use crate::proxy::{proxy_handler, ServiceAppState};
use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post, put},
    Router,
};
use jwt_common::{auth, require_user};
use std::sync::Arc;

pub fn router(state: Arc<ServiceAppState>) -> Router<Arc<ServiceAppState>> {
    let jwt_config = state.jwt_config.clone();

    let public_routes = Router::new()
        .route("/", get(proxy_handler))
        .route("/games/recent", get(proxy_handler))
        .route("/{id}", get(proxy_handler));

    let authenticated_routes = Router::new()
        .route("/", post(proxy_handler))
        .route("/me/{id}", get(proxy_handler))
        .route("/{id}", put(proxy_handler))
        .route("/{id}", delete(proxy_handler))
        .route_layer(from_fn(require_user))
        .route_layer(from_fn_with_state(jwt_config, auth));

    Router::new()
        .merge(public_routes)
        .merge(authenticated_routes)
}
