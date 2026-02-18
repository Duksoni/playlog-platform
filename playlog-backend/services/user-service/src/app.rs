use crate::{
    auth::{router as auth_router, AuthService},
    config::AppConfig,
    users::{router as users_router, UserService},
};
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method,
        StatusCode,
    },
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use std::{sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer, trace::TraceLayer};
use utoipa::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_swagger_ui::SwaggerUi;

pub struct AppState {
    pub config: AppConfig,
    pub auth_service: AuthService,
    pub user_service: UserService,
}

impl AppState {
    pub fn new(config: AppConfig, auth_service: AuthService, user_service: UserService) -> Self {
        Self {
            config,
            auth_service,
            user_service,
        }
    }
}


pub fn build_app(state: Arc<AppState>) -> Router {
    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    let (router, api) = OpenApiRouter::with_openapi(crate::docs::ApiDoc::openapi())
        .route("/user-service-health", get(health_check))
        .nest("/auth", auth_router())
        .nest("/users", users_router(Arc::clone(&state)))
        .layer((
            TraceLayer::new_for_http(),
            TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10)),
        ))
        .layer(cors.clone())
        .with_state(Arc::clone(&state))
        .split_for_parts();

    Router::new()
        .route("/", get(root_redirect))
        .nest("/api", router)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api.clone()))
}

#[utoipa::path(
    get,
    path = "/api/user-service-health",
    summary = "API Health check",
    responses(
        (status = 200, description = "Health check passed"),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "user_service_health",
    operation_id = "user_service_health"
)]
async fn health_check() -> Result<impl IntoResponse, StatusCode> {
    Ok((StatusCode::OK, "API is healthy!".into_response()))
}

async fn root_redirect() -> Redirect {
    Redirect::permanent("/docs")
}
