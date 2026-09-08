use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Serialize, ToSchema)]
pub struct GameMediaResponse {
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub cover: Option<MediaFileResponse>,
    pub screenshots: Vec<MediaFileResponse>,
    pub trailer: Option<MediaFileResponse>,
    pub version: i64,
}

impl GameMediaResponse {
    pub fn new(
        game_id: i32,
        cover: Option<MediaFileResponse>,
        screenshots: Vec<MediaFileResponse>,
        trailer: Option<MediaFileResponse>,
        version: i64,
    ) -> Self {
        Self {
            game_id,
            cover,
            screenshots,
            trailer,
            version,
        }
    }
}

#[derive(Debug, Serialize, ToSchema)]
pub struct MediaFileResponse {
    pub url: String,
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    #[serde(rename = "sizeBytes")]
    pub size_bytes: usize,
}

impl MediaFileResponse {
    pub fn new(url: String, mime_type: String, size_bytes: usize) -> Self {
        Self {
            url,
            mime_type,
            size_bytes,
        }
    }
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct DeleteGameMediaRequest {
    pub version: i64,
}

#[derive(Debug, Validate, Deserialize, IntoParams)]
pub struct GetGameCoversQuery {
    #[serde(default, rename = "gameIds")]
    #[validate(length(max = 100))]
    pub game_ids: Vec<i32>,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct GetGameCoversResponse {
    #[serde(rename = "gameCovers")]
    pub game_covers: HashMap<i32, Option<String>>,
}

impl GetGameCoversResponse {
    pub fn new(game_covers: HashMap<i32, Option<String>>) -> Self {
        Self { game_covers }
    }

    pub fn empty() -> Self {
        Self {
            game_covers: Default::default(),
        }
    }
}
