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

use crate::{
    app::{build_app, AppState},
    config::load_from_environment,
    repository::PostgresLibraryRepository,
    service::LibraryService,
    setup::{init_db, init_tracing, shutdown_signal},
};
use dotenvy::dotenv;
use reqwest::Client;
use std::{net::SocketAddr, sync::Arc, time::Duration};
use tracing::info;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    init_tracing();

    let env = load_from_environment()?;

    let pool = init_db(&env.database_url).await?;

    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .expect("Failed to create HTTP client");

    let repository = Box::new(PostgresLibraryRepository::new(pool));
    let library_service = LibraryService::new(
        repository,
        client,
        env.app_config.catalogue_service_url.clone(),
    );

    let state = Arc::new(AppState {
        config: env.app_config,
        library_service,
    });

    let app = build_app(state);

    let server_address = SocketAddr::from(([0, 0, 0, 0], 3002));
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
