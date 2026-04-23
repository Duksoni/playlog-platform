use crate::proxy::proxy_handler;
use axum::{routing::get, Router};
use std::sync::Arc;
use crate::proxy::ServiceAppState;

pub fn router() -> Router<Arc<ServiceAppState>> {
    Router::new().route("/review-service-health", get(proxy_handler))
}
