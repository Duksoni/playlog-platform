use crate::proxy::{proxy_handler, ServiceAppState};
use axum::{
    routing::get,
    Router,
};
use std::sync::Arc;

pub fn router() -> Router<Arc<ServiceAppState>> {
    Router::new().route(
        "/catalogue-service-health",
        get(proxy_handler),
    )
}
