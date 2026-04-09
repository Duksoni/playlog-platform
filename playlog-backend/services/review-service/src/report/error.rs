use api_error::ApiError;
use axum::http::StatusCode;
use mongodb::bson::oid::ObjectId;
use thiserror::Error;
use tracing::error;

#[derive(Error, Debug)]
pub enum ReportError {
    #[error("Illegal status: {0}")]
    IllegalStatus(String),

    #[error("Report not found")]
    NotFound,

    #[error("Database error: {0}")]
    DatabaseError(#[from] mongodb::error::Error),

    #[error("Version mismatch for report with id {0}")]
    Conflict(ObjectId),

    #[error("{0}")]
    AnyhowError(#[from] anyhow::Error),
}

pub type Result<T> = std::result::Result<T, ReportError>;

impl From<ReportError> for ApiError {
    fn from(error: ReportError) -> Self {
        let status = match &error {
            ReportError::IllegalStatus(_) | ReportError::AnyhowError(_) => StatusCode::BAD_REQUEST,
            ReportError::NotFound => StatusCode::NOT_FOUND,
            ReportError::Conflict(_) => StatusCode::CONFLICT,
            ReportError::DatabaseError(db_err) => {
                error!(error = %db_err, "database error");
                return ApiError::internal_error()
            }
        };
        ApiError::new(status, error.to_string())
    }
}
