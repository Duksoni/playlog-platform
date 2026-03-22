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
        let url = format!("{}{}", service_url, path);

        // Convert axum Body to bytes
        let body_bytes = axum::body::to_bytes(body, usize::MAX).await.map_err(|e| {
            GatewayError::RequestFailed(format!("Failed to read request body: {}", e))
        })?;

        // Build the proxied request
        let mut request = self.client.request(method.clone(), &url);

        // Forward relevant headers (especially Authorization for double verification)
        for (name, value) in headers.iter() {
            // Skip certain headers that should not be forwarded
            let name_str = name.as_str();
            if name_str.starts_with("host") || name_str.starts_with("content-length") {
                continue;
            }
            request = request.header(name, value);
        }

        // Add body if present
        if !body_bytes.is_empty() {
            request = request.body(body_bytes.to_vec());
        }

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
        for (name, value) in headers.iter() {
            builder = builder.header(name, value);
        }

        let response = builder
            .body(Body::from(body_bytes))
            .map_err(|_| GatewayError::InvalidResponse)?;

        Ok(response)
    }
}
