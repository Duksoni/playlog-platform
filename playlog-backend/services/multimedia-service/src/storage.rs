use crate::error::{MediaError, Result};
use async_trait::async_trait;
use axum::http::Method;
use bytes::Bytes;
use futures::StreamExt;
use minio::s3::{
    client::Client as MinioClient,
    multimap::{Multimap, MultimapExt},
    segmented_bytes::SegmentedBytes,
    types::{S3Api, ToStream},
};
use tracing::warn;

#[async_trait]
pub trait MediaStorage: Send + Sync {
    async fn put_object(&self, key: &str, content_type: String, data: Bytes) -> Result<()>;
    async fn delete_keys(&self, keys: &[String]);
    async fn list_prefix_keys(&self, prefix: &str) -> Result<Vec<String>>;
    async fn presign(&self, object_key: &str) -> Result<String>;
}

#[derive(Debug, Clone)]
pub struct MinioMediaStorage {
    client: MinioClient,
    bucket: String,
}

impl MinioMediaStorage {
    pub fn new(client: MinioClient, bucket: String) -> Self {
        Self { client, bucket }
    }
}

#[async_trait]
impl MediaStorage for MinioMediaStorage {
    async fn put_object(&self, key: &str, content_type: String, data: Bytes) -> Result<()> {
        let segmented = SegmentedBytes::from(data);
        let mut extra_headers = Multimap::new();
        extra_headers.add("Content-Type", content_type);

        self.client
            .put_object(&self.bucket, key, segmented)
            .extra_headers(Some(extra_headers))
            .send()
            .await
            .map_err(|e| MediaError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn delete_keys(&self, keys: &[String]) {
        for key in keys {
            if let Err(error) = self
                .client
                .delete_object(&self.bucket, key.clone())
                .send()
                .await
            {
                warn!(object_key = %key, error = %error, "failed to delete media object");
            }
        }
    }

    async fn list_prefix_keys(&self, prefix: &str) -> Result<Vec<String>> {
        let mut stream = self
            .client
            .list_objects(&self.bucket)
            .prefix(Some(prefix.to_string()))
            .recursive(true)
            .to_stream()
            .await;

        let mut keys = Vec::new();
        while let Some(listed) = stream.next().await {
            let response = listed.map_err(|e| MediaError::StorageError(e.to_string()))?;
            keys.extend(
                response
                    .contents
                    .into_iter()
                    .filter(|entry| !entry.is_prefix)
                    .map(|entry| entry.name),
            );
            if keys.len() >= 1000 {
                break;
            }
        }
        keys.truncate(1000);
        Ok(keys)
    }

    async fn presign(&self, object_key: &str) -> Result<String> {
        self.client
            .get_presigned_object_url(&self.bucket, object_key, Method::GET)
            .expiry_seconds(60 * 60)
            .send()
            .await
            .map(|response| response.url)
            .map_err(|e| MediaError::StorageError(e.to_string()))
    }
}
