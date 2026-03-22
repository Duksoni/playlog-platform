use crate::proxy::{proxy_handler, ServiceAppState};
use axum::{
    routing::post,
    Router,
};
use std::sync::Arc;

pub fn router() -> Router<Arc<ServiceAppState>> {
    Router::new()
        .route("/login", post(proxy_handler))
        .route("/register", post(proxy_handler))
        .route("/logout", post(proxy_handler))
        .route("/refresh", post(proxy_handler))
}
