use axum::{
    http::{
        header::{ACCEPT, AUTHORIZATION, CONTENT_TYPE}, HeaderValue, Method,
        StatusCode,
    },
    response::Redirect,
};
use std::time::Duration;
use tower_http::{cors::CorsLayer, timeout::TimeoutLayer};

pub fn cors_layer(allow_put: bool) -> CorsLayer {
    let mut methods = vec![Method::GET, Method::POST, Method::DELETE];
    if allow_put {
        methods.push(Method::PUT);
    }
    CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_origin("http://localhost:8080".parse::<HeaderValue>().unwrap())
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
