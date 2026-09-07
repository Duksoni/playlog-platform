use crate::model::GameMedia;
use anyhow::Context;
use minio::s3::{
    client::{Client as MinioClient, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
};
use mongodb::{bson::doc, options::IndexOptions, Collection, IndexModel};

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

pub async fn create_indexes(collection: &Collection<GameMedia>) -> anyhow::Result<()> {
    let game_id_unique = IndexModel::builder()
        .keys(doc! { "game_id": 1 })
        .options(IndexOptions::builder().unique(true).build())
        .build();
    collection.create_index(game_id_unique).await?;
    Ok(())
}
