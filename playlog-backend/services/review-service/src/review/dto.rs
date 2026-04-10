use super::{Rating, Review};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct CreateUpdateReviewRequest {
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub rating: Rating,
    #[validate(length(min = 10))]
    pub text: Option<String>,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct ReviewQuery {
    #[param(required = false, example = "1")]
    pub page: u64,
    pub rating: Option<Rating>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GameReviewResponse {
    pub id: String,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub username: String,
    pub rating: Rating,
    pub text: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl From<Review> for GameReviewResponse {
    fn from(value: Review) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            user_id: value.user_id,
            username: value.username,
            rating: value.rating,
            text: value.text,
            created_at: value.created_at.to_chrono(),
            updated_at: value.updated_at.to_chrono(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GameRatingStatsResponse {
    #[serde(rename = "highlyRecommendedCount")]
    pub highly_recommended_count: i64,
    #[serde(rename = "goodCount")]
    pub good_count: i64,
    #[serde(rename = "okayCount")]
    pub okay_count: i64,
    #[serde(rename = "notRecommendedCount")]
    pub not_recommended_count: i64,
}

impl GameRatingStatsResponse {
    pub fn new(
        highly_recommended_count: i64,
        good_count: i64,
        okay_count: i64,
        not_recommended_count: i64,
    ) -> Self {
        Self {
            highly_recommended_count,
            good_count,
            okay_count,
            not_recommended_count,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReviewSimpleResponse {
    pub id: String,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub username: String,
    pub rating: Rating,
    pub text: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
}

impl From<Review> for ReviewSimpleResponse {
    fn from(value: Review) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            game_id: value.game_id,
            user_id: value.user_id,
            username: value.username,
            rating: value.rating,
            text: value.text,
            created_at: value.created_at.to_chrono(),
            updated_at: value.updated_at.to_chrono(),
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ReviewDetailedResponse {
    pub id: String,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    pub username: String,
    pub rating: Rating,
    pub text: Option<String>,
    #[serde(rename = "createdAt")]
    pub created_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub updated_at: DateTime<Utc>,
    pub version: i64,
}

impl From<Review> for ReviewDetailedResponse {
    fn from(value: Review) -> Self {
        Self {
            id: value.id.unwrap().to_string(),
            game_id: value.game_id,
            user_id: value.user_id,
            username: value.username,
            rating: value.rating,
            text: value.text,
            created_at: value.created_at.to_chrono(),
            updated_at: value.updated_at.to_chrono(),
            version: value.version,
        }
    }
}
