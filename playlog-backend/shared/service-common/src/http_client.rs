use std::time::Duration;
use anyhow::anyhow;
use axum::http::StatusCode;
use reqwest::Client as HttpClient;

pub fn build_client() -> HttpClient {
    HttpClient::builder()
        .timeout(Duration::from_secs(30))
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
