use crate::proxy::{proxy_handler, ServiceAppState};
use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post, put},
    Router,
};
use jwt_common::{auth, require_admin};
use std::sync::Arc;

pub fn router(state: Arc<ServiceAppState>) -> Router<Arc<ServiceAppState>> {
    let jwt_config = state.jwt_config.clone();

    let public_routes = Router::new()
        .route("/filter", get(proxy_handler))
        .route("/{id}", get(proxy_handler))
        .route("/{id}/details", get(proxy_handler))
        .route("/by-developer/{id}", get(proxy_handler))
        .route("/by-publisher/{id}", get(proxy_handler));

    let admin_routes = Router::new()
        .route("/", post(proxy_handler))
        .route("/unpublished", get(proxy_handler))
        .route("/{id}", put(proxy_handler))
        .route("/{id}", delete(proxy_handler))
        .route("/{id}/publish", put(proxy_handler))
        .route("/{id}/unpublish", put(proxy_handler))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn_with_state(jwt_config, auth));

    Router::new()
        .merge(public_routes)
        .merge(admin_routes)
}
