use mongodb::bson::{DateTime, oid::ObjectId};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum CommentTargetType {
    Game,
    Review,
}

impl CommentTargetType {
    pub fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Comment {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub target_type: CommentTargetType,
    pub target_id: String, // Can be game_id or review_id
    pub user_id: Uuid,
    pub username: String,
    pub text: String,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub version: i64,
    pub deleted: bool,
}

impl Comment {
    pub fn new(
        target_type: CommentTargetType,
        target_id: String,
        user_id: Uuid,
        username: String,
        text: String,
        created_at: DateTime,
    ) -> Self {
        Self {
            id: None,
            target_type,
            target_id,
            user_id,
            username,
            text,
            created_at,
            updated_at: created_at,
            version: 0,
            deleted: false,
        }
    }
}
