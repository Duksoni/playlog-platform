use crate::error::{GatewayError, Result};
use axum::{
    body::Body,
    http::{HeaderMap, Method},
    response::Response,
};
use reqwest::Client;

#[derive(Clone)]
pub struct ProxyClient {
    client: Client,
}

const HOP_BY_HOP_HEADERS: &[&str] = &[
    "connection",
    "keep-alive",
    "proxy-authenticate",
    "proxy-authorization",
    "te",
    "trailer",
    "transfer-encoding",
    "upgrade",
];

const REGENERATED_HEADERS: &[&str] = &["host", "content-length"];

fn should_forward_header(header_name: &str) -> bool {
    !HOP_BY_HOP_HEADERS
        .iter()
        .chain(REGENERATED_HEADERS.iter())
        .any(|blocked| blocked.eq_ignore_ascii_case(header_name))
}

impl ProxyClient {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    /// Forward a request to a backend service
    ///
    /// This method:
    /// - Preserves the original Authorization header (for double verification)
    /// - Forwards the request method, path, headers, and body
    /// - Returns the backend service's response as-is
    pub async fn forward_request(
        &self,
        service_url: &str,
        path: &str,
        method: Method,
        headers: HeaderMap,
        body: Body,
    ) -> Result<Response> {
        let base = service_url.trim_end_matches('/');
        let url = format!("{}{}", base, path);

        // Build the proxied request
        let mut request = self.client.request(method.clone(), &url);

        // Forward relevant headers (especially Authorization for double verification)
        for (header_name, header_value) in headers
            .iter()
            .filter(|(header_name, _)| should_forward_header(header_name.as_str()))
        {
            request = request.header(header_name, header_value);
        }

        // Add body if present - stream it without reading it all into memory
        request = request.body(reqwest::Body::wrap_stream(body.into_data_stream()));

        // Send the request
        let response = request.send().await.map_err(|e| {
            GatewayError::ServiceUnavailable(format!("Failed to reach service: {}", e))
        })?;

        // Convert reqwest::Response to axum::Response
        Self::convert_response(response).await
    }

    async fn convert_response(response: reqwest::Response) -> Result<Response> {
        let status = response.status();
        let headers = response.headers().clone();
        let body_bytes = response
            .bytes()
            .await
            .map_err(|_| GatewayError::InvalidResponse)?;

        // Build axum response
        let mut builder = Response::builder().status(status);

        // Copy headers from the backend response
        for (header_name, header_value) in headers
            .iter()
            .filter(|(header_name, _)| should_forward_header(header_name.as_str()))
        {
            builder = builder.header(header_name, header_value);
        }

        let response = builder
            .body(Body::from(body_bytes))
            .map_err(|_| GatewayError::InvalidResponse)?;

        Ok(response)
    }
}
