use anyhow::{Context, Result};
use std::{env::var, fs::read};

pub struct Environment {
    pub mongodb_uri: String,
    pub mongodb_database: String,
    pub reviews_collection: String,
    pub comments_collection: String,
    pub reports_collection: String,
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
    let mongodb_uri = var("MONGODB_URI").context("MONGODB_URI not set")?;
    let mongodb_database = var("MONGODB_DATABASE").context("MONGODB_DATABASE not set")?;
    let reviews_collection = var("REVIEWS_COLLECTION").unwrap_or_else(|_| "reviews".to_string());
    let comments_collection = var("COMMENTS_COLLECTION").unwrap_or_else(|_| "comments".to_string());
    let reports_collection = var("REPORTS_COLLECTION").unwrap_or_else(|_| "reports".to_string());

    let app_config = AppConfig::from_env()?;
    Ok(Environment {
        mongodb_uri,
        mongodb_database,
        reviews_collection,
        comments_collection,
        reports_collection,
        app_config,
    })
}
