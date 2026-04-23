use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::{FromRow, Type};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize, Type, ToSchema, Hash)]
#[sqlx(type_name = "game_library_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum GameLibraryStatus {
    #[default]
    Owned,
    Playing,
    Wishlist,
    Completed,
    Dropped,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct UserGame {
    #[serde(rename = "userId")]
    pub user_id: Uuid,
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub status: GameLibraryStatus,
    #[serde(rename = "addedAt")]
    pub added_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Serialize, FromRow, ToSchema)]
pub struct LibraryGame {
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub status: GameLibraryStatus,
    #[serde(rename = "addedAt")]
    pub added_at: DateTime<Utc>,
    #[serde(rename = "updatedAt")]
    pub last_updated: DateTime<Utc>,
}