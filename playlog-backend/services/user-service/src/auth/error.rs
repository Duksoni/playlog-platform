use api_error::ApiError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AuthError {
    #[error("Invalid credentials")]
    InvalidCredentials,

    #[error("User not found")]
    UserNotFound,

    #[error("Username is already taken")]
    UsernameTaken,

    #[error("Email already in use")]
    EmailAlreadyExists,

    #[error("User blocked")]
    UserBlocked,

    #[error("Token expired")]
    TokenExpired,

    #[error("Internal error")]
    InternalError,
}

pub type Result<T> = std::result::Result<T, AuthError>;

use AuthError::*;

impl From<AuthError> for ApiError {
    fn from(error: AuthError) -> Self {
        let status_code: StatusCode = match error {
            InvalidCredentials | TokenExpired => StatusCode::UNAUTHORIZED,
            UserBlocked => StatusCode::FORBIDDEN,
            UserNotFound => StatusCode::NOT_FOUND,
            UsernameTaken | EmailAlreadyExists => StatusCode::CONFLICT,
            InternalError => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError::new(status_code, error.to_string())
    }
}
