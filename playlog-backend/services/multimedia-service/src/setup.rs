use anyhow::Context;
use minio::s3::{
    client::{Client as MinioClient, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
};
use mongodb::{Client as MongoClient, Collection};
use tokio::signal;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::model::GameMedia;

pub fn init_tracing() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
                format!(
                    "{}=debug,tower_http=debug,axum=trace",
                    env!("CARGO_CRATE_NAME")
                )
                .into()
            }),
        )
        .with(tracing_subscriber::fmt::layer().without_time())
        .init();
}

pub async fn init_db(
    mongodb_uri: &str,
    database: &str,
    collection: &str,
) -> anyhow::Result<Collection<GameMedia>> {
    let client = MongoClient::with_uri_str(mongodb_uri)
        .await
        .context("Failed to connect to MongoDB")?;

    let collection = client
        .database(database)
        .collection::<GameMedia>(collection);
    Ok(collection)
}

pub fn init_minio(
    endpoint: &str,
    access_key: &str,
    secret_key: &str,
) -> anyhow::Result<MinioClient> {
    let base_url: BaseUrl = endpoint.parse().context("Invalid MinIO endpoint URL")?;

    let credentials = StaticProvider::new(access_key, secret_key, None);

    let client = ClientBuilder::new(base_url)
        .provider(Some(credentials))
        .build()
        .context("Failed to build MinIO client")?;

    Ok(client)
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
