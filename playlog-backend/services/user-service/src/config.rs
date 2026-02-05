use std::env::var;

pub struct Config {
    pub jwt_secret: String,
    pub access_token_expiration_seconds: u16,
    pub refresh_token_expiration_days: u8,
}

impl Config {
    pub fn from_env() -> anyhow::Result<Self> {
        Ok(Self {
            jwt_secret: var("JWT_SECRET").expect("JWT secret not set"),
            access_token_expiration_seconds: var("ACCESS_TOKEN_EXPIRATION_SECONDS")
                .unwrap_or(String::from("300"))
                .parse()?,
            refresh_token_expiration_days: var("REFRESH_TOKEN_EXPIRATION_DAYS")
                .unwrap_or(String::from("14"))
                .parse()?,
        })
    }
}
