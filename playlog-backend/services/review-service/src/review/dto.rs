use super::{Rating, Review};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateUpdateReviewRequest {
    pub game_id: i32,
    pub rating: Rating,
    #[validate(length(min = 10))]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ReviewQuery {
    pub page: Option<u64>,
    pub rating: Option<Rating>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GameReviewResponse {
    pub id: String,
    pub user_id: Uuid,
    pub rating: Rating,
    pub text: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<Review> for GameReviewResponse {
    fn from(value: Review) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            user_id: value.user_id,
            rating: value.rating,
            text: value.text,
            updated_at: value.updated_at.to_chrono(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReviewSimpleResponse {
    pub id: String,
    pub game_id: i32,
    pub user_id: Uuid,
    pub rating: Rating,
    pub text: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl From<Review> for ReviewSimpleResponse {
    fn from(value: Review) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            game_id: value.game_id,
            user_id: value.user_id,
            rating: value.rating,
            text: value.text,
            updated_at: value.updated_at.to_chrono(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReviewDetailedResponse {
    pub id: String,
    pub game_id: i32,
    pub user_id: Uuid,
    pub rating: Rating,
    pub text: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

impl From<Review> for ReviewDetailedResponse {
    fn from(value: Review) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            game_id: value.game_id,
            user_id: value.user_id,
            rating: value.rating,
            text: value.text,
            created_at: value.created_at.to_chrono(),
            updated_at: value.updated_at.to_chrono(),
            version: value.version,
        }
    }
}
