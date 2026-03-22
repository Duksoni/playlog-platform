use crate::proxy::{proxy_handler, ServiceAppState};
use axum::{
    middleware::{from_fn, from_fn_with_state},
    routing::{delete, get, post},
    Router,
};
use jwt_common::{auth, require_admin};
use std::sync::Arc;

pub fn router(state: Arc<ServiceAppState>) -> Router<Arc<ServiceAppState>> {
    let jwt_config = state.jwt_config.clone();

    let public_routes = Router::new().route("/games/{game_id}", get(proxy_handler));

    let admin_routes = Router::new()
        .route("/games/{game_id}/upload", post(proxy_handler))
        .route("/games/{game_id}", delete(proxy_handler))
        .route_layer(from_fn(require_admin))
        .route_layer(from_fn_with_state(jwt_config.clone(), auth));

    Router::new()
        .merge(public_routes)
        .merge(admin_routes)
}
