use service_common::error::ApiError;
use axum::http::StatusCode;
use bson::oid::ObjectId;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum CommentError {
    #[error("Comment not found")]
    NotFound,

    #[error("Invalid game id: {0}")]
    InvalidGameId(String),

    #[error("Invalid review id: {0}")]
    InvalidReviewId(String),

    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Version mismatch for comment with id {0}")]
    Conflict(ObjectId),
}

pub type Result<T> = std::result::Result<T, CommentError>;

impl From<CommentError> for ApiError {
    fn from(error: CommentError) -> Self {
        let status = match &error {
            CommentError::InvalidGameId(_)
            | CommentError::InvalidReviewId(_)
            | CommentError::AnyhowError(_) => StatusCode::BAD_REQUEST,
            CommentError::Unauthorized => StatusCode::UNAUTHORIZED,
            CommentError::NotFound => StatusCode::NOT_FOUND,
            CommentError::Conflict(_) => StatusCode::CONFLICT,
            CommentError::DatabaseError(db_err) => {
                error!(error = %db_err, "database error");
                return ApiError::internal_error()
            }
        };
        ApiError::new(status, error.to_string())
    }
}
