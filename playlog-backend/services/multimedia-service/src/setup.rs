use anyhow::Context;
use minio::s3::{
    client::{Client as MinioClient, ClientBuilder},
    creds::StaticProvider,
    http::BaseUrl,
};

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
