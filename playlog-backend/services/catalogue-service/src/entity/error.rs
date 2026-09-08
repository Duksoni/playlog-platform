use axum::http::StatusCode;
use service_common::error::{
    is_foreign_key_violation, is_row_not_found, is_unique_violation, ApiError,
};
use thiserror::Error;
use tracing::error;

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
            GameEntityError::DatabaseError(db_err) => {
                if is_row_not_found(&db_err) {
                    return ApiError::new(StatusCode::NOT_FOUND, "Entity not found");
                }
                if is_unique_violation(&db_err) || is_foreign_key_violation(&db_err) {
                    return ApiError::new(StatusCode::CONFLICT, db_err.to_string());
                }
                error!(error = %db_err, "database error");
                return ApiError::internal_error();
            }
            GameEntityError::UnsupportedOperation(_, _) => StatusCode::NOT_IMPLEMENTED,
            GameEntityError::Conflict(_, _) => StatusCode::CONFLICT,
        };

        ApiError::new(status, error.to_string())
    }
}
