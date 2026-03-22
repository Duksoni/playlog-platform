use crate::entity::GameEntityError;
use api_error::ApiError;
use axum::http::StatusCode;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum GameError {
    #[error("Game with id {0} not found")]
    NotFound(i32),

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
            GameError::EntityError(_) => StatusCode::BAD_REQUEST,
            GameError::DatabaseError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };
        ApiError::new(status_code, error.to_string())
    }
}
