use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportTargetType {
    Review,
    Comment,
}

impl ReportTargetType {
    pub fn as_db_value(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ReportStatus {
    Pending,
    Resolved,
    Dismissed,
}

impl ReportStatus {
    pub fn as_db_value(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Report {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub target_type: ReportTargetType,
    pub target_id: ObjectId, // Either review or comment
    pub reporter_id: Uuid,
    pub reporter_username: String,
    pub reason: String,
    pub status: ReportStatus,
    pub created_at: DateTime,
    pub version: i64,
}

impl Report {
    pub fn new(
        target_type: ReportTargetType,
        target_id: ObjectId,
        reporter_id: Uuid,
        reporter_username: String,
        reason: String,
        created_at: DateTime,
    ) -> Self {
        Self {
            id: None,
            target_type,
            target_id,
            reporter_id,
            reporter_username,
            reason,
            status: ReportStatus::Pending,
            created_at,
            version: 0,
        }
    }
}
