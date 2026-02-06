use anyhow::{Context, Result};
use std::env::var;

pub type DBUrl = String;
pub struct Environment(pub DBUrl, pub AppConfig, pub Option<AdminBootstrapConfig>);

pub struct AppConfig {
    pub jwt_secret: String,
    pub access_token_expiration_seconds: u16,
    pub refresh_token_expiration_days: u8,
}

impl AppConfig {
    fn from_env() -> Result<Self> {
        Ok(Self {
            jwt_secret: var("JWT_SECRET").context("JWT_SECRET must be set")?,
            access_token_expiration_seconds: var("ACCESS_TOKEN_EXPIRATION_SECONDS")
                .map(|val| val.parse::<u16>())
                .unwrap_or(Ok(300))?,
            refresh_token_expiration_days: var("REFRESH_TOKEN_EXPIRATION_DAYS")
                .map(|val| val.parse::<u8>())
                .unwrap_or(Ok(30))?,
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
            let email = var("ADMIN_EMAIL")
                .context("ADMIN_EMAIL must be set")?;
            let username = var("ADMIN_USERNAME")
                .context("ADMIN_USERNAME must be set")?;
            let temp_password = var("ADMIN_TEMP_PASSWORD")
                .context("ADMIN_TEMP_PASSWORD must be set")?;

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

    Ok(Environment(
        database_url,
        app_config,
        admin_bootstrap_config,
    ))
}
