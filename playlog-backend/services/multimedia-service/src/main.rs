use axum::{
    routing::get,
    Router,
};
mod config;
mod dto;
mod error;
mod model;
mod repository;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello from Mulitimedia Service!" }));

    // run our app with hyper, listening globally on port 3003
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3003").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}