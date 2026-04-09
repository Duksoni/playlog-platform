use api_error::ApiError;
use axum::http::StatusCode;
use mongodb::bson::oid::ObjectId;
use thiserror::Error;
use tracing::error;

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Review not found")]
    NotFound,

    #[error("Game with ID {0} does not exist")]
    InvalidGameId(i32),

    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Version mismatch for review with id {0}")]
    Conflict(ObjectId),
}

pub type Result<T> = std::result::Result<T, ReviewError>;

impl From<ReviewError> for ApiError {
    fn from(error: ReviewError) -> Self {
        let status = match &error {
            ReviewError::InvalidGameId(_) | ReviewError::AnyhowError(_) => StatusCode::BAD_REQUEST,
            ReviewError::Unauthorized => StatusCode::FORBIDDEN,
            ReviewError::Conflict(_) => StatusCode::CONFLICT,
            ReviewError::NotFound => StatusCode::NOT_FOUND,
            ReviewError::DatabaseError(db_err) => {
                error!(error = %db_err, "database error");
                return ApiError::internal_error();
            }
        };
        ApiError::new(status, error.to_string())
    }
}
