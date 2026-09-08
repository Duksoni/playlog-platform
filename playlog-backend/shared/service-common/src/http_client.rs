use std::time::Duration;
use anyhow::anyhow;
use axum::http::StatusCode;
use reqwest::Client as HttpClient;

pub fn build_client() -> HttpClient {
    build_client_with_timeout(Duration::from_secs(30))
}

pub fn build_client_with_timeout(timeout: Duration) -> HttpClient {
    HttpClient::builder()
        .timeout(timeout)
        .build()
        .expect("Failed to create HTTP client")
}

pub async fn expect_ok_get_response(
    client: &HttpClient,
    endpoint: &str,
    error_message: &str,
) -> anyhow::Result<()> {
    let response = client.get(endpoint).send().await?;
    if response.status() == StatusCode::OK {
        Ok(())
    } else {
        Err(anyhow!("{}", error_message))
    }
}

#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum CatalogueError {
    #[error("Game with ID {0} does not exist")]
    NotFound(i32),
    #[error("Catalogue service error: {0}")]
    Unavailable(String),
}

#[derive(Debug, Clone)]
pub struct CatalogueClient {
    http: HttpClient,
    base_url: String,
}

impl CatalogueClient {
    pub fn new(http: HttpClient, base_url: String) -> Self {
        Self {
            http,
            base_url: base_url.trim_end_matches('/').to_string(),
        }
    }

    pub async fn ensure_game_exists(&self, game_id: i32) -> Result<(), CatalogueError> {
        let url = format!("{}/api/games/{}", self.base_url, game_id);
        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|error| CatalogueError::Unavailable(error.to_string()))?;
        match response.status() {
            StatusCode::OK => Ok(()),
            StatusCode::NOT_FOUND => Err(CatalogueError::NotFound(game_id)),
            status => Err(CatalogueError::Unavailable(format!(
                "Unexpected status from catalogue service: {status}"
            ))),
        }
    }
}
