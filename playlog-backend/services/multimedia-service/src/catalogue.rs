use crate::error::{MediaError, Result};
use reqwest::Client as HttpClient;

#[derive(Debug, Clone)]
pub struct CatalogueClient {
    http: HttpClient,
    base_url: String,
}

impl CatalogueClient {
    pub fn new(http: HttpClient, base_url: String) -> Self {
        Self { http, base_url }
    }

    pub async fn ensure_game_exists(&self, game_id: i32) -> Result<()> {
        let url = format!("{}/api/games/{}", self.base_url, game_id);

        let response = self
            .http
            .get(url)
            .send()
            .await
            .map_err(|e| MediaError::CatalogueServiceError(e.to_string()))?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::NOT_FOUND => Err(MediaError::InvalidGameId(game_id)),
            _ => Err(MediaError::CatalogueServiceError(format!(
                "Unexpected status from catalogue service: {}",
                response.status()
            ))),
        }
    }
}
