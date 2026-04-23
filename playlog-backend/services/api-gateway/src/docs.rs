use crate::config::Config;
use anyhow::{Context, Result};
use reqwest::Client;
use serde_json::from_str;
use tracing::error;
use utoipa::{openapi::OpenApi as OpenApiDocument, OpenApi};

pub const OPENAPI_DOC_PATH: &str = "/api-docs/openapi.json";

#[derive(OpenApi)]
#[openapi(
    info(title = "Playlog API Gateway", version = "0.1.0"),
    paths(crate::app::health_check)
)]
struct ApiDoc;

pub async fn load_service_docs(config: &Config, client: &Client) -> OpenApiDocument {
    let mut merged = ApiDoc::openapi();

    let service_urls: Vec<&str> = vec![
        &config.user_service_url,
        &config.multimedia_service_url,
        &config.catalogue_service_url,
        &config.library_service_url,
        &config.reviews_service_url,
    ];

    for base_url in service_urls {
        match fetch_service_doc(client, base_url).await {
            Ok(spec) => merged.merge(spec),
            Err(err) => error!("Failed to load OpenAPI spec from {}: {err}", base_url),
        }
    }

    merged
}

async fn fetch_service_doc(client: &Client, base_url: &str) -> Result<OpenApiDocument> {
    let doc_url = format!("{base_url}{OPENAPI_DOC_PATH}");
    let res = client
        .get(doc_url.clone())
        .send()
        .await
        .with_context(|| format!("request to {doc_url} failed"))?
        .error_for_status()
        .with_context(|| format!("{doc_url} returned non-success status"))?;

    let text = res
        .text()
        .await
        .context("failed to read OpenAPI response body")?;

    from_str(&text).context("failed to deserialize OpenAPI JSON")
}
