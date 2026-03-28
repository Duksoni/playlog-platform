use api_error::ApiError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameEntityError {
    #[error("{0} with id {1} not found")]
    NotFound(String, i32),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),

    #[error("Operation {1} is not supported for {0}")]
    UnsupportedOperation(String, String),

    #[error("Conflict: Version mismatch for {0} with id {1}")]
    Conflict(String, i32),
}

pub type Result<T> = std::result::Result<T, GameEntityError>;

impl From<GameEntityError> for ApiError {
    fn from(error: GameEntityError) -> Self {
        let status = match error {
            GameEntityError::NotFound(_, _) => StatusCode::NOT_FOUND,
            GameEntityError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            GameEntityError::UnsupportedOperation(_, _) => StatusCode::NOT_IMPLEMENTED,
            GameEntityError::Conflict(_, _) => StatusCode::CONFLICT,
        };

        ApiError::new(status, error.to_string())
    }
}
