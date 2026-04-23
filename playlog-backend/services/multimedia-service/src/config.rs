use anyhow::{Context, Result};
use std::{env::var, fs::read};

pub struct Environment {
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub mongodb_collection: String,
    pub minio_server_url: String,
    pub minio_access_key: String,
    pub minio_secret_key: String,
    pub minio_bucket: String,
    pub app_config: AppConfig,
}

pub struct AppConfig {
    pub jwt_public_key: Vec<u8>,
    pub catalogue_service_url: String,
}

impl AppConfig {
    fn from_env() -> Result<Self> {
        let public_key_path =
            var("JWT_PUBLIC_KEY_PATH").context("JWT_PUBLIC_KEY_PATH must be set")?;
        let jwt_public_key = read(&public_key_path)
            .with_context(|| format!("failed to read {}", public_key_path))?;
        let catalogue_service_url =
            var("CATALOGUE_SERVICE_URL").context("CATALOGUE_SERVICE_URL must be set")?;
        Ok(Self {
            jwt_public_key,
            catalogue_service_url,
        })
    }
}

pub fn load_from_environment() -> Result<Environment> {
    let mongodb_uri = var("MONGODB_URI").context("MongoDB URI not set")?;
    let mongodb_database = var("MONGODB_DATABASE").context("MongoDB database not set")?;
    let mongodb_collection = var("MONGODB_COLLECTION").context("MongoDB collection not set")?;
    let minio_endpoint = var("MINIO_SERVER_URL").context("Minio endpoint not set")?;
    let minio_access_key = var("MINIO_ACCESS_KEY").context("Minio access key not set")?;
    let minio_secret_key = var("MINIO_SECRET_KEY").context("Minio secret key not set")?;
    let minio_bucket = var("MINIO_BUCKET").context("MINIO_BUCKET must be set")?;
    let app_config = AppConfig::from_env()?;
    Ok(Environment {
        mongodb_uri,
        mongodb_database,
        mongodb_collection,
        minio_server_url: minio_endpoint,
        minio_access_key,
        minio_secret_key,
        minio_bucket,
        app_config,
    })
}
