use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GatewayError {
    #[error("Service unavailable: {0}")]
    ServiceUnavailable(String),

    #[error("Invalid response from service")]
    InvalidResponse,
}

pub type Result<T> = std::result::Result<T, GatewayError>;

use GatewayError::*;

impl IntoResponse for GatewayError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            ServiceUnavailable(msg) => (StatusCode::SERVICE_UNAVAILABLE, msg),
            InvalidResponse => (
                StatusCode::BAD_GATEWAY,
                "Invalid response from backend service".to_string(),
            ),
        };

        let body = Json(json!({
            "error": message,
        }));

        (status, body).into_response()
    }
}
