use crate::{comment::CommentError, review::ReviewError};
use service_common::error::ApiError;
use axum::http::StatusCode;
use mongodb::bson::oid::ObjectId;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Illegal status: {0}")]
    IllegalStatus(String),

    #[error("You can't resolve your own report")]
    CantResolveOwnReport,

    #[error("Report not found")]
    NotFound,

    #[error("You have already reported this content and it is still under review")]
    AlreadyReported,

    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Version mismatch for report with id {0}")]
    Conflict(ObjectId),

    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),

    #[error("Review error: {0}")]
    ReviewError(#[from] ReviewError),

    #[error("Comment error: {0}")]
    CommentError(#[from] CommentError),
}

pub type Result<T> = std::result::Result<T, ReportError>;

impl From<ReportError> for ApiError {
    fn from(error: ReportError) -> Self {
        match error {
            ReportError::ReviewError(err) => ApiError::from(err),
            ReportError::CommentError(err) => ApiError::from(err),
            _ => {
                let status = match &error {
                    ReportError::IllegalStatus(_)
                    | ReportError::CantResolveOwnReport
                    | ReportError::AnyhowError(_) => StatusCode::BAD_REQUEST,
                    ReportError::NotFound => StatusCode::NOT_FOUND,
                    ReportError::Conflict(_) | ReportError::AlreadyReported => StatusCode::CONFLICT,
                    ReportError::DatabaseError(db_err) => {
                        error!(error = %db_err, "database error");
                        return ApiError::internal_error();
                    }
                    _ => StatusCode::INTERNAL_SERVER_ERROR,
                };
                ApiError::new(status, error.to_string())
            }
        }
    }
}
