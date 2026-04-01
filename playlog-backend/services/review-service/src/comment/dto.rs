use super::{Comment, CommentTargetType};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateCommentRequest {
    pub target_type: CommentTargetType,
    #[validate(length(min = 1))]
    pub target_id: String,
    #[validate(length(min = 10))]
    pub text: String,
}

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct UpdateCommentRequest {
    #[validate(length(min = 10))]
    pub text: String,
}

#[derive(Debug, Validate, Deserialize, IntoParams)]
pub struct CommentQuery {
    pub target_type: CommentTargetType,
    #[validate(length(min = 1))]
    pub target_id: String,
    pub page: Option<u64>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct SimpleCommentResponse {
    pub id: String,
    pub user_id: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

impl From<Comment> for SimpleCommentResponse {
    fn from(value: Comment) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            user_id: value.user_id,
            text: value.text,
            created_at: value.created_at.to_chrono(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct DetailedCommentResponse {
    pub id: String,
    pub target_type: CommentTargetType,
    pub target_id: String,
    pub user_id: Uuid,
    pub text: String,
    pub created_at: DateTime<Utc>,
}

impl From<Comment> for DetailedCommentResponse {
    fn from(value: Comment) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            target_type: value.target_type,
            target_id: value.target_id,
            user_id: value.user_id,
            text: value.text,
            created_at: value.created_at.to_chrono(),
        }
    }
}
