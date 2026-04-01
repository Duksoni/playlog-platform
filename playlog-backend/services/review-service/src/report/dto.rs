use super::{Report, ReportStatus, ReportTargetType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateReportRequest {
    pub target_type: ReportTargetType,
    pub target_id: String,
    #[validate(length(min = 10))]
    pub reason: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct UpdateReportStatusRequest {
    pub status: ReportStatus,
    pub version: i64,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ReportQuery {
    pub page: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReportResponse {
    pub id: String,
    pub target_type: ReportTargetType,
    pub target_id: String,
    pub reporter_id: Uuid,
    pub reason: String,
    pub created_at: DateTime<Utc>,
    pub version: i64,
}

impl From<Report> for ReportResponse {
    fn from(value: Report) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            target_type: value.target_type,
            target_id: value.target_id.to_string(),
            reporter_id: value.reporter_id,
            reason: value.reason,
            created_at: value.created_at.to_chrono(),
            version: value.version,
        }
    }
}
