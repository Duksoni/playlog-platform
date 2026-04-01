use api_error::ApiError;
use axum::http::StatusCode;
use mongodb::bson::oid::ObjectId;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ReviewError {
    #[error("Review not found")]
    NotFound,

    #[error("Game with ID {0} does not exist")]
    InvalidGameId(i32),

    #[error("Database error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Version mismatch for review with id {0}")]
    Conflict(ObjectId),

    #[error("Internal error")]
    InternalError,
}

pub type Result<T> = std::result::Result<T, ReviewError>;

impl From<ReviewError> for ApiError {
    fn from(error: ReviewError) -> Self {
        let status = match &error {
            ReviewError::InvalidGameId(_) | ReviewError::AnyhowError(_) => StatusCode::BAD_REQUEST,
            ReviewError::Unauthorized => StatusCode::FORBIDDEN,
            ReviewError::Conflict(_) => StatusCode::CONFLICT,
            ReviewError::NotFound => StatusCode::NOT_FOUND,
            ReviewError::MongoError(_) | ReviewError::InternalError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        };
        ApiError::new(status, error.to_string())
    }
}
