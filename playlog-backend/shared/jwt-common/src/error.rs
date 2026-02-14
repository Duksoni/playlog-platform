use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwtError {
    #[error("Missing authorization header")]
    MissingAuthorization,

    #[error("Authorization header error: {0}")]
    InvalidAuthorizationHeader(String),

    #[error("Invalid decoding key: {0}")]
    InvalidDecodingKey(String),

    #[error("Invalid token: {0}")]
    InvalidToken(String),

    #[error("Missing claims in request")]
    MissingClaims,

    #[error("Insufficient role permissions")]
    Forbidden,
}

pub type Result<T> = std::result::Result<T, JwtError>;

use JwtError::*;

impl IntoResponse for JwtError {
    fn into_response(self) -> Response {
        let status_code = match self {
            MissingClaims
            | MissingAuthorization
            | InvalidAuthorizationHeader(_)
            | InvalidToken(_) => StatusCode::UNAUTHORIZED,
            Forbidden => StatusCode::FORBIDDEN,
            InvalidDecodingKey(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        (status_code, self.to_string()).into_response()
    }
}
