use crate::{config::AppConfig, docs::ApiDoc, handler::router, service::MediaService};
use axum::{
    extract::DefaultBodyLimit, http::StatusCode, response::IntoResponse, routing::get, Router,
};
use service_common::app::{cors_layer, root_redirect};
use std::{sync::Arc, time::Duration};
use tower_http::{timeout::TimeoutLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub struct AppState {
    pub config: AppConfig,
    pub media_service: MediaService,
}

impl AppState {
    pub fn new(config: AppConfig, media_service: MediaService) -> Self {
        Self {
            config,
            media_service,
        }
    }
}

pub fn build_app(state: Arc<AppState>) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/multimedia-service-health", get(health_check))
        .nest("/media", router(Arc::clone(&state)))
        .layer((
            TraceLayer::new_for_http(),
            // Longer timeout for multimedia uploads (10 minutes)
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(600)),
        ))
        .layer(DefaultBodyLimit::max(512 * 1024 * 1024)) // 512 MB
        .layer(cors_layer(false))
        .with_state(Arc::clone(&state))
        .split_for_parts();

    Router::new()
        .route("/", get(root_redirect))
        .nest("/api", router)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api))
}

#[utoipa::path(
    get,
    path = "/api/multimedia-service-health",
    summary = "API Health check",
    responses(
        (status = 200, description = "Health check passed"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "multimedia_service_health",
    operation_id = "multimedia_service_health"
)]
pub async fn health_check() -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::OK, "API is healthy!".into_response()))
}
