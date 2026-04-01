use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Rating {
    NotRecommended,
    Okay,
    Good,
    HighlyRecommended,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Review {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String)]
    pub id: Option<ObjectId>,
    pub game_id: i32,
    pub user_id: Uuid,
    pub rating: Rating,
    pub text: Option<String>,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    pub version: i64,
    pub deleted: bool,
}

impl Review {
    pub fn new(
        game_id: i32,
        user_id: Uuid,
        rating: Rating,
        text: Option<String>,
        created_at: DateTime,
    ) -> Self {
        Self {
            id: None,
            game_id,
            user_id,
            rating,
            text,
            created_at,
            updated_at: created_at,
            version: 0,
            deleted: false,
        }
    }
}
