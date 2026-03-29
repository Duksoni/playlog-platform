use crate::{config::AppConfig, docs::ApiDoc, handler::router, service::LibraryService};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use service_common::app::{cors_layer, root_redirect, timeout_layer};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub struct AppState {
    pub config: AppConfig,
    pub library_service: LibraryService,
}

pub fn build_app(state: Arc<AppState>) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/library-service-health", get(health_check))
        .nest("/library", router(Arc::clone(&state)))
        .layer((TraceLayer::new_for_http(), timeout_layer()))
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
    path = "/api/library-service-health",
    summary = "API Health check",
    responses(
        (status = 200, description = "Health check passed"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "library_service_health",
    operation_id = "library_service_health"
)]
pub async fn health_check() -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::OK, "API is healthy!".into_response()))
}
