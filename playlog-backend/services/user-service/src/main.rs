mod app;
mod auth;
mod config;

use axum::{
    routing::get,
    Router,
};

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello from User Service!" }));

    // run our app with hyper, listening globally on port 3005
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3005").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}