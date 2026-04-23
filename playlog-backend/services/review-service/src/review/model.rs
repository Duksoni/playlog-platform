use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use thiserror::Error;
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

impl Rating {
    pub fn as_db_value(&self) -> String {
        format!("{:?}", self).to_uppercase()
    }
}

#[derive(Error, Debug)]
#[error("invalid rating: {0}")]
pub struct RatingParseError(String);

impl FromStr for Rating {
    type Err = RatingParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "NOT_RECOMMENDED" => Ok(Rating::NotRecommended),
            "OKAY" => Ok(Rating::Okay),
            "GOOD" => Ok(Rating::Good),
            "HIGHLY_RECOMMENDED" => Ok(Rating::HighlyRecommended),
            other => Err(RatingParseError(String::from(other))),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct Review {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[schema(value_type = String)]
    pub id: Option<ObjectId>,
    pub game_id: i32,
    pub user_id: Uuid,
    pub username: String,
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
        username: String,
        rating: Rating,
        text: Option<String>,
        created_at: DateTime,
    ) -> Self {
        Self {
            id: None,
            game_id,
            user_id,
            username,
            rating,
            text,
            created_at,
            updated_at: created_at,
            version: 0,
            deleted: false,
        }
    }
}
