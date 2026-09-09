use axum::{
    extract::DefaultBodyLimit,
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method,
        StatusCode,
    },
    response::Redirect,
    routing::get,
    Router,
};
use std::time::Duration;
use tower_http::{
    cors::{AllowOrigin, CorsLayer},
    normalize_path::NormalizePathLayer,
    timeout::TimeoutLayer,
};
use utoipa::openapi::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

pub const DEFAULT_BODY_LIMIT_BYTES: usize = 2 * 1024 * 1024;

pub fn cors_layer(allow_put: bool) -> CorsLayer {
    let mut methods = vec![Method::GET, Method::POST, Method::DELETE];
    if allow_put {
        methods.push(Method::PUT);
    }
    CorsLayer::new()
        .allow_origin(AllowOrigin::list([
            "http://localhost:4200".parse::<HeaderValue>().unwrap(),
            "http://localhost:8080".parse::<HeaderValue>().unwrap(),
        ]))
        .allow_headers([AUTHORIZATION, ACCEPT, CONTENT_TYPE])
        .allow_credentials(true)
        .allow_methods(methods)
}

pub fn timeout_layer() -> TimeoutLayer {
    TimeoutLayer::with_status_code(StatusCode::REQUEST_TIMEOUT, Duration::from_secs(10))
}

pub async fn root_redirect() -> Redirect {
    Redirect::permanent("/docs")
}

pub fn finalize_router(api_router: Router, api: OpenApi) -> Router {
    finalize_router_with_body_limit(api_router, api, Some(DEFAULT_BODY_LIMIT_BYTES))
}

pub fn finalize_router_with_body_limit(
    api_router: Router,
    api: OpenApi,
    limit_bytes: Option<usize>,
) -> Router {
    let mut router = Router::new()
        .route("/", get(root_redirect))
        .nest("/api", api_router)
        .merge(SwaggerUi::new("/docs").url("/api-docs/openapi.json", api));
    if let Some(limit) = limit_bytes {
        router = router.layer(DefaultBodyLimit::max(limit));
    }
    router.layer(NormalizePathLayer::trim_trailing_slash())
}
