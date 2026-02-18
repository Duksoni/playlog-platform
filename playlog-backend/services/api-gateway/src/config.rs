use anyhow::{Context, Result};
use std::{env::var, fs::read};

#[derive(Clone)]
pub struct Config {
    pub jwt_public_key: Vec<u8>,
    pub catalogue_service_url: String,
    pub library_service_url: String,
    pub multimedia_service_url: String,
    pub reviews_service_url: String,
    pub user_service_url: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let public_key_path =
            var("JWT_PUBLIC_KEY_PATH").context("JWT_PUBLIC_KEY_PATH must be set")?;
        let jwt_public_key = read(&public_key_path)
            .with_context(|| format!("Failed to read JWT public key from {}", public_key_path))?;
        let catalogue_service_url =
            var("CATALOGUE_SERVICE_URL").context("CATALOGUE_SERVICE_URL must be set")?;
        let library_service_url =
            var("LIBRARY_SERVICE_URL").context("LIBRARY_SERVICE_URL must be set")?;
        let multimedia_service_url =
            var("MULTIMEDIA_SERVICE_URL").context("MULTIMEDIA_SERVICE_URL must be set")?;
        let reviews_service_url =
            var("REVIEW_SERVICE_URL").context("REVIEW_SERVICE_URL must be set")?;
        let user_service_url = var("USER_SERVICE_URL").context("USER_SERVICE_URL must be set")?;

        Ok(Self {
            jwt_public_key,
            catalogue_service_url,
            library_service_url,
            multimedia_service_url,
            reviews_service_url,
            user_service_url,
        })
    }
}
