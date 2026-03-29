mod app;
mod config;
mod docs;
mod dto;
mod error;
mod handler;
mod model;
mod repository;
mod service;
mod setup;

use dotenvy::dotenv;
use service_common::setup::{init_mongodb, init_tracing, shutdown_signal};
use std::net::SocketAddr;
use std::sync::Arc;
use tracing::info;

use app::{build_app, AppState};
use repository::MongoMediaRepository;
use service::MediaService;
use setup::init_minio;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    init_tracing(env!("CARGO_CRATE_NAME"));

    let env = config::load_from_environment()?;

    let mongodb_client = init_mongodb(&env.mongodb_uri).await?;
    let collection = mongodb_client
        .database(&env.mongodb_database)
        .collection(&env.mongodb_collection);

    let minio = init_minio(
        &env.minio_server_url,
        &env.minio_access_key,
        &env.minio_secret_key,
    )?;

    let http_client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client");

    let repository = Box::new(MongoMediaRepository::new(collection));
    let media_service = MediaService::new(
        repository,
        minio,
        env.minio_bucket,
        http_client,
        env.app_config.catalogue_service_url.clone(),
    );

    let state = Arc::new(AppState::new(env.app_config, media_service));
    let app = build_app(state);

    let server_address = SocketAddr::from(([0, 0, 0, 0], 3003));
    let listener = tokio::net::TcpListener::bind(&server_address).await?;

    info!(
        "Server started. View docs at http://{}/docs",
        server_address
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;

    Ok(())
}
