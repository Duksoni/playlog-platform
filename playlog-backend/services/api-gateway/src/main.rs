mod app;
mod config;
mod docs;
mod error;
mod proxy;

use dotenvy::dotenv;
use service_common::setup::{init_tracing, shutdown_signal};
use std::net::SocketAddr;
use tracing::info;

use app::build_app;
use config::Config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    init_tracing(env!("CARGO_CRATE_NAME"));

    let config = Config::from_env()?;

    let app = build_app(config).await;

    let server_address = SocketAddr::from(([0, 0, 0, 0], 3000));
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
