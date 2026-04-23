use crate::entity::GameEntityError;
use service_common::error::ApiError;
use axum::http::StatusCode;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Game with id {0} not found")]
    NotFound(i32),

    #[error("Conflict: Version mismatch for game with id {0}")]
    Conflict(i32),

    #[error("No ids provided for field {0}")]
    NoIdsProvided(String),

    #[error("Entity error: {0}")]
    EntityError(#[from] GameEntityError),

    #[error("Database error: {0}")]
    DatabaseError(#[from] sqlx::Error),
}

pub type Result<T> = std::result::Result<T, GameError>;

impl From<GameError> for ApiError {
    fn from(error: GameError) -> Self {
        let status_code = match error {
            GameError::NotFound(_) => StatusCode::NOT_FOUND,
            GameError::Conflict(_) => StatusCode::CONFLICT,
            GameError::NoIdsProvided(_) | GameError::EntityError(_) => StatusCode::BAD_REQUEST,
            GameError::DatabaseError(db_err) => {
                error!(error = %db_err, "database error");
                return ApiError::internal_error()
            }
        };
        ApiError::new(status_code, error.to_string())
    }
}
