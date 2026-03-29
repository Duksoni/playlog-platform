use anyhow::Context;
use mongodb::Client as MongoClient;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

pub fn init_tracing(crate_name: &str) {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace",
                    crate_name
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();
}

pub async fn shutdown_signal() {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

pub async fn init_sqlx_db(connection_string: &str) -> anyhow::Result<PgPool> {
    let pool = PgPoolOptions::new()
        .connect(connection_string)
        .await
        .context("Failed to connect to DB")?;
    Ok(pool)
}

pub async fn init_mongodb(mongodb_uri: &str) -> anyhow::Result<MongoClient> {
    let client = MongoClient::with_uri_str(mongodb_uri)
        .await
        .context("Failed to connect to MongoDB")?;
    Ok(client)
}
