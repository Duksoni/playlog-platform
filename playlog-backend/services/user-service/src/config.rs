use anyhow::{Context, Result};
use chrono::Duration;
use std::{
    env::var,
    fs::read
};

pub struct Environment {
    pub database_url: String,
    pub app_config: AppConfig,
    pub admin_bootstrap_config: Option<AdminBootstrapConfig>,
}

pub struct AppConfig {
    pub jwt_private_key: Vec<u8>,
    pub jwt_public_key: Vec<u8>,
    pub access_token_validity: Duration,
    pub refresh_token_validity: Duration,
}

impl AppConfig {
    fn from_env() -> Result<Self> {
        let private_key_path =
            var("JWT_PRIVATE_KEY_PATH").context("JWT_PRIVATE_KEY_PATH must be set")?;
        let jwt_private_key = read(&private_key_path)
            .with_context(|| format!("failed to read {}", private_key_path))?;
        let public_key_path =
            var("JWT_PUBLIC_KEY_PATH").context("JWT_PUBLIC_KEY_PATH must be set")?;
        let jwt_public_key = read(&public_key_path)
            .with_context(|| format!("failed to read {}", public_key_path))?;
        let access_token_exp = var("ACCESS_TOKEN_VALIDITY_SECONDS")
            .map(|val| val.parse::<u16>())
            .unwrap_or(Ok(300))?;
        let refresh_token_exp = var("REFRESH_TOKEN_VALIDITY_DAYS")
            .map(|val| val.parse::<u8>())
            .unwrap_or(Ok(14))?;
        Ok(Self {
            jwt_private_key,
            jwt_public_key,
            access_token_validity: Duration::seconds(access_token_exp as i64),
            refresh_token_validity: Duration::days(refresh_token_exp as i64),
        })
    }
}

pub struct AdminBootstrapConfig {
    pub email: String,
    pub username: String,
    pub temp_password: String,
}

impl AdminBootstrapConfig {
    fn from_env() -> Result<Option<Self>> {
        let enabled = var("ADMIN_BOOTSTRAP_ENABLED")
            .map(|val| val.parse::<bool>())
            .unwrap_or(Ok(false))?;

        if enabled {
            let email = var("ADMIN_EMAIL").context("ADMIN_EMAIL must be set")?;
            let username = var("ADMIN_USERNAME").context("ADMIN_USERNAME must be set")?;
            let temp_password =
                var("ADMIN_TEMP_PASSWORD").context("ADMIN_TEMP_PASSWORD must be set")?;

            return Ok(Some(Self {
                email,
                username,
                temp_password,
            }));
        }
        Ok(None)
    }
}

pub fn load_from_environment() -> Result<Environment> {
    let database_url = var("DATABASE_URL").context("Database URL not set")?;
    let app_config = AppConfig::from_env()?;
    let admin_bootstrap_config = AdminBootstrapConfig::from_env()?;

    Ok(Environment {
        database_url,
        app_config,
        admin_bootstrap_config,
    })
}
