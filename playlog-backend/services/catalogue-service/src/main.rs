use axum::{
    routing::get,
    Router,
};
mod developers;
mod entity;
mod games;
mod genres;
mod platforms;
mod publishers;
mod tags;

#[tokio::main]
async fn main() {
    // build our application with a single route
    let app = Router::new().route("/", get(|| async { "Hello from Catalogue Service!" }));

    // run our app with hyper, listening globally on port 3001
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3001").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}