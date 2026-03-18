use anyhow::{Context, Result};
use std::{env::var, fs::read};

pub struct Environment {
    pub database_url: String,
    pub app_config: AppConfig,
}

pub struct AppConfig {
    pub jwt_public_key: Vec<u8>,
}

impl AppConfig {
    fn from_env() -> Result<Self> {
        let public_key_path =
            var("JWT_PUBLIC_KEY_PATH").context("JWT_PUBLIC_KEY_PATH must be set")?;
        let jwt_public_key = read(&public_key_path)
            .with_context(|| format!("failed to read {}", public_key_path))?;
        Ok(Self { jwt_public_key })
    }
}

pub fn load_from_environment() -> Result<Environment> {
    let database_url = var("DATABASE_URL").context("Database URL not set")?;
    let app_config = AppConfig::from_env()?;
    Ok(Environment {
        database_url,
        app_config,
    })
}
