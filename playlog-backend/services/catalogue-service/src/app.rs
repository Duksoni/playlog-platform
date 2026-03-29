use crate::{
    config::AppConfig,
    developers::handler::router as developers_router,
    docs::ApiDoc,
    entity::{GameEntityRepository, SmallGameEntityRepository},
    games::{handler::router as games_router, GameService},
    genres::handler::router as genres_router,
    platforms::handler::router as platforms_router,
    publishers::handler::router as publishers_router,
    tags::handler::router as tags_router,
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
    pub game_service: GameService,
    pub developer_repository: Arc<dyn GameEntityRepository>,
    pub genre_repository: Arc<dyn SmallGameEntityRepository>,
    pub platform_repository: Arc<dyn SmallGameEntityRepository>,
    pub publisher_repository: Arc<dyn GameEntityRepository>,
    pub tag_repository: Arc<dyn GameEntityRepository>,
}

impl AppState {
    pub fn new(
        config: AppConfig,
        game_service: GameService,
        developer_repository: Arc<dyn GameEntityRepository>,
        genre_repository: Arc<dyn SmallGameEntityRepository>,
        platform_repository: Arc<dyn SmallGameEntityRepository>,
        publisher_repository: Arc<dyn GameEntityRepository>,
        tag_repository: Arc<dyn GameEntityRepository>,
    ) -> Self {
        Self {
            config,
            game_service,
            developer_repository,
            genre_repository,
            platform_repository,
            publisher_repository,
            tag_repository,
        }
    }
}

pub fn build_app(state: Arc<AppState>) -> Router {
    let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
        .route("/catalogue-service-health", get(health_check))
        .nest("/games", games_router(Arc::clone(&state)))
        .nest("/developers", developers_router(Arc::clone(&state)))
        .nest("/genres", genres_router(Arc::clone(&state)))
        .nest("/platforms", platforms_router(Arc::clone(&state)))
        .nest("/publishers", publishers_router(Arc::clone(&state)))
        .nest("/tags", tags_router(Arc::clone(&state)))
        .layer((TraceLayer::new_for_http(), timeout_layer()))
        .layer(cors_layer(true))
        .with_state(Arc::clone(&state))
        .split_for_parts();

    Router::new()
        .route("/", get(root_redirect))
        .nest("/api", router)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api))
}

#[utoipa::path(
    get,
    path = "/api/catalogue-service-health",
    summary = "API Health check",
    responses(
        (status = 200, description = "Health check passed"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "catalogue_service_health",
    operation_id = "catalogue_service_health"
)]
pub async fn health_check() -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::OK, "API is healthy!".into_response()))
}
