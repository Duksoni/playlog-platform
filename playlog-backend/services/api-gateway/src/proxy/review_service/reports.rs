use crate::proxy::{proxy_handler, ServiceAppState};
use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{get, post, put},
    Router,
};
use jwt_common::{auth, require_moderator, require_user};
use std::sync::Arc;

pub fn router(state: Arc<ServiceAppState>) -> Router<Arc<ServiceAppState>> {
    let jwt_config = state.jwt_config.clone();

    let user_routes = Router::new()
        .route("/", post(proxy_handler))
        .route_layer(from_fn(require_user));

    let moderator_routes = Router::new()
        .route("/", get(proxy_handler))
        .route("/pending", get(proxy_handler))
        .route("/{id}/status", put(proxy_handler))
        .route_layer(from_fn(require_moderator));

    Router::new()
        .merge(user_routes)
        .merge(moderator_routes)
        .route_layer(from_fn_with_state(jwt_config, auth))
}
