mod app;
mod config;
mod docs;
mod dto;
mod error;
mod handler;
mod model;
mod repository;
mod service;

use anyhow::Context;
use dotenvy::dotenv;
use service_common::{
    http_client::build_client,
    setup::{init_sqlx_db, init_tracing, shutdown_signal},
};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

use crate::{
    app::{build_app, AppState},
    config::load_from_environment,
    repository::PostgresLibraryRepository,
    service::LibraryService,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    init_tracing(env!("CARGO_CRATE_NAME"));

    let env = load_from_environment()?;

    let pool = init_sqlx_db(&env.database_url).await?;

    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Migrations failed")?;

    let http_client = build_client();

    let repository = Box::new(PostgresLibraryRepository::new(pool));
    let library_service = LibraryService::new(
        repository,
        http_client,
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
