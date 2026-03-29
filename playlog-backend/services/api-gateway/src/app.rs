use crate::{
    config::Config,
    docs::{load_service_docs, OPENAPI_DOC_PATH},
    proxy::{
        catalogue_service::{catalogue_health_router, entity_router, games_router}, library_service::{library_health_router, library_router},
        multimedia_service::{multimedia_health_router, multimedia_router},
        user_service::{auth_router, users_health_router, users_router},
        ProxyClient,
        ServiceAppState,
    },
};
use axum::{http::StatusCode, response::IntoResponse, routing::get, Router};
use jwt_common::JwtConfig;
use service_common::{
    app::{cors_layer, root_redirect},
    http_client::build_client,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;
use utoipa_swagger_ui::SwaggerUi;

pub async fn build_app(config: Config) -> Router {
    let client = build_client();

    let docs = load_service_docs(&config, &client).await;
    let proxy_client = ProxyClient::new(client.clone());
    let jwt_config = JwtConfig::new(config.jwt_public_key.clone());

    let user_app_state = Arc::new(ServiceAppState::new(
        config.user_service_url.clone(),
        proxy_client.clone(),
        jwt_config.clone(),
    ));

    let multimedia_app_state = Arc::new(ServiceAppState::new(
        config.multimedia_service_url.clone(),
        proxy_client.clone(),
        jwt_config.clone(),
    ));

    let library_app_state = Arc::new(ServiceAppState::new(
        config.library_service_url.clone(),
        proxy_client.clone(),
        jwt_config.clone(),
    ));

    let catalogue_app_state = Arc::new(ServiceAppState::new(
        config.catalogue_service_url.clone(),
        proxy_client.clone(),
        jwt_config,
    ));

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
            "/api",
            multimedia_health_router().with_state(Arc::clone(&multimedia_app_state)),
        )
        .nest(
            "/api",
            catalogue_health_router().with_state(Arc::clone(&catalogue_app_state)),
        )
        .nest(
            "/api",
            library_health_router().with_state(Arc::clone(&library_app_state)),
        )
        .nest(
            "/api/auth",
            auth_router().with_state(Arc::clone(&user_app_state)),
        )
        .nest(
            "/api/users",
            users_router(Arc::clone(&user_app_state)).with_state(Arc::clone(&user_app_state)),
        )
        .nest(
            "/api/media",
            multimedia_router(Arc::clone(&multimedia_app_state))
                .with_state(Arc::clone(&multimedia_app_state)),
        )
        .nest(
            "/api/games",
            games_router(Arc::clone(&catalogue_app_state))
                .with_state(Arc::clone(&catalogue_app_state)),
        )
        .nest(
            "/api/library",
            library_router(Arc::clone(&library_app_state))
                .with_state(Arc::clone(&library_app_state)),
        )
        .nest(
            "/api/developers",
            entity_router(Arc::clone(&catalogue_app_state), "developers")
                .with_state(Arc::clone(&catalogue_app_state)),
        )
        .nest(
            "/api/genres",
            entity_router(Arc::clone(&catalogue_app_state), "genres")
                .with_state(Arc::clone(&catalogue_app_state)),
        )
        .nest(
            "/api/platforms",
            entity_router(Arc::clone(&catalogue_app_state), "platforms")
                .with_state(Arc::clone(&catalogue_app_state)),
        )
        .nest(
            "/api/publishers",
            entity_router(Arc::clone(&catalogue_app_state), "publishers")
                .with_state(Arc::clone(&catalogue_app_state)),
        )
        .nest(
            "/api/tags",
            entity_router(Arc::clone(&catalogue_app_state), "tags")
                .with_state(Arc::clone(&catalogue_app_state)),
        )
        .merge(swagger_ui)
        .layer(TraceLayer::new_for_http())
        .layer(cors_layer(true))
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
