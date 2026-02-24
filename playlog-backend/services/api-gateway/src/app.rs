use crate::docs::{load_service_docs, OPENAPI_DOC_PATH};
use crate::proxy::user_service::{auth_router, users_health_router, users_router, UserAppState};
use crate::{config::Config, proxy::ProxyClient};
use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method,
        StatusCode,
    },
    response::{IntoResponse, Redirect},
    routing::get,
    Router,
};
use jwt_common::JwtConfig;
use reqwest::Client;
use std::{sync::Arc, time::Duration};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use utoipa_swagger_ui::SwaggerUi;

pub async fn build_app(config: Config) -> Router {
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client");

    let docs = load_service_docs(&config, &client).await;
    let proxy_client = ProxyClient::new(client.clone());
    let jwt_config = JwtConfig::new(config.jwt_public_key.clone());

    let user_app_state = Arc::new(UserAppState::new(
        config.user_service_url.clone(),
        proxy_client.clone(),
        jwt_config,
    ));

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE]);

    // Swagger UI with merged OpenAPI spec
    let swagger_ui = SwaggerUi::new("/docs").url(OPENAPI_DOC_PATH, docs);

    Router::new()
        .route("/", get(root_redirect))
        .route("/api/health", get(health_check))
        .nest(
            "/api",
            users_health_router().with_state(Arc::clone(&user_app_state)),
        )
        .nest(
            "/api/auth",
            auth_router().with_state(Arc::clone(&user_app_state)),
        )
        .nest(
            "/api/users",
            users_router(Arc::clone(&user_app_state)).with_state(Arc::clone(&user_app_state)),
        )
        .merge(swagger_ui)
        .layer(TraceLayer::new_for_http())
        .layer(cors)
}

#[utoipa::path(
    get,
    path = "/api/health",
    responses(
        (status = 200, description = "API Gateway is healthy")
    ),
    tag = "gateway_health",
    operation_id = "gateway_health"
)]
async fn health_check() -> impl IntoResponse {
    (StatusCode::OK, "API is healthy!")
}

async fn root_redirect() -> Redirect {
    Redirect::permanent("/docs")
}
