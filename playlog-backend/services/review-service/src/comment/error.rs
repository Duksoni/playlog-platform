use api_error::ApiError;
use axum::http::StatusCode;
use bson::oid::ObjectId;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CommentError {
    #[error("Comment not found")]
    NotFound,

    #[error("Invalid game id: {0}")]
    InvalidGameId(String),

    #[error("Invalid review id: {0}")]
    InvalidReviewId(String),

    #[error("Database error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Version mismatch for comment with id {0}")]
    Conflict(ObjectId),

    #[error("Internal error")]
    InternalError,
}

pub type Result<T> = std::result::Result<T, CommentError>;

impl From<CommentError> for ApiError {
    fn from(error: CommentError) -> Self {
        let status = match &error {
            CommentError::InvalidGameId(_)
            | CommentError::InvalidReviewId(_)
            | CommentError::AnyhowError(_) => StatusCode::BAD_REQUEST,
            CommentError::Conflict(_) => StatusCode::CONFLICT,
            CommentError::NotFound => StatusCode::NOT_FOUND,
            CommentError::MongoError(_) | CommentError::InternalError => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
            CommentError::Unauthorized => StatusCode::UNAUTHORIZED,
        };
        ApiError::new(status, error.to_string())
    }
}
