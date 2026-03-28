use serde::Serialize;
use utoipa::ToSchema;

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
