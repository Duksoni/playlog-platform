use crate::error::MediaError;
use bytes::Bytes;
use mongodb::bson::{oid::ObjectId, DateTime};
use serde::{Deserialize, Serialize};
use std::str::FromStr;

#[derive(Debug, Serialize, Deserialize)]
pub struct GameMedia {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub game_id: i32,
    pub cover: Option<MediaFile>,
    pub screenshots: Vec<MediaFile>,
    pub trailer: Option<MediaFile>,
}

impl GameMedia {
    pub fn new(
        id: Option<ObjectId>,
        game_id: i32,
        cover: Option<MediaFile>,
        screenshots: Vec<MediaFile>,
        trailer: Option<MediaFile>,
    ) -> Self {
        Self {
            id,
            game_id,
            cover,
            screenshots,
            trailer,
        }
    }

    pub fn new_for_game(game_id: i32) -> Self {
        Self::new(None, game_id, None, vec![], None)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MediaFile {
    pub object_key: String,
    pub mime_type: String,
    pub size_bytes: usize,
    pub uploaded_at: DateTime,
}

impl MediaFile {
    pub fn new(
        object_key: String,
        mime_type: String,
        size_bytes: usize,
        uploaded_at: DateTime,
    ) -> Self {
        Self {
            object_key,
            mime_type,
            size_bytes,
            uploaded_at,
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum FieldName {
    Cover,
    Screenshot,
    Trailer,
}

impl FieldName {
    pub fn as_string(&self) -> String {
        format!("{:?}", self)
    }
}

impl FromStr for FieldName {
    type Err = MediaError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cover" => Ok(Self::Cover),
            "screenshot" => Ok(Self::Screenshot),
            "trailer" => Ok(Self::Trailer),
            _ => Err(MediaError::UnknownField(s.to_string())),
        }
    }
}

pub struct UploadedFile {
    pub field_name: FieldName,
    pub file_name: String,
    pub content_type: String,
    pub data: Bytes,
}

impl UploadedFile {
    pub fn new(
        field_name: FieldName,
        file_name: String,
        content_type: String,
        data: Bytes,
    ) -> Self {
        Self {
            field_name,
            file_name,
            content_type,
            data,
        }
    }
}
