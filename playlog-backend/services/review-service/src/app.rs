use crate::{
    comment::{handler::router as comment_router, CommentService},
    config::AppConfig,
    docs::ApiDoc,
    report::{handler::router as report_router, ReportService},
    review::{handler::router as review_router, ReviewService},
};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use service_common::app::{cors_layer, root_redirect, timeout_layer};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub struct AppState {
    pub config: AppConfig,
    pub comment_service: CommentService,
    pub report_service: ReportService,
    pub review_service: ReviewService,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        comment_service: CommentService,
        report_service: ReportService,
        review_service: ReviewService,
    ) -> Self {
        Self {
            config,
            comment_service,
            report_service,
            review_service,
        }
    }
}

pub fn build_app(state: Arc<AppState>) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/review-service-health", get(health_check))
        .nest("/reviews", review_router(Arc::clone(&state)))
        .nest("/comments", comment_router(Arc::clone(&state)))
        .nest("/reports", report_router(Arc::clone(&state)))
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
    path = "/api/review-service-health",
    summary = "API Health check",
    responses(
        (status = 200, description = "Health check passed"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "review_service_health",
    operation_id = "review_service_health"
)]
pub async fn health_check() -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::OK, "API is healthy!".into_response()))
}
