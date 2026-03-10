use axum::http::Method;
use bytes::Bytes;
use minio::s3::{
    client::Client as MinioClient,
    multimap::{Multimap, MultimapExt},
    segmented_bytes::SegmentedBytes,
    types::S3Api,
};
use mongodb::bson::DateTime;
use std::time::SystemTime;

use crate::{
    dto::{GameMediaResponse, MediaFileResponse},
    error::{MediaError, Result},
    model::{
        FieldName::{self, *},
        GameMedia, MediaFile, UploadedFile,
    },
    repository::MediaRepository,
};

const MAX_IMAGE_BYTES: usize = 10 * 1024 * 1024; // 10 MB
const MAX_VIDEO_BYTES: usize = 500 * 1024 * 1024; // 500 MB

pub struct MediaService {
    repository: Box<dyn MediaRepository>,
    minio: MinioClient,
    bucket: String,
}

impl MediaService {
    pub fn new(repository: Box<dyn MediaRepository>, minio: MinioClient, bucket: String) -> Self {
        Self {
            repository,
            minio,
            bucket,
        }
    }

    pub async fn get_game_media(&self, game_id: i32) -> Result<GameMediaResponse> {
        let media = self.find_by_game_id(game_id).await?;
        self.to_response(media).await
    }

    pub async fn upload_game_media(
        &self,
        game_id: i32,
        files: Vec<UploadedFile>,
    ) -> Result<GameMediaResponse> {
        self.validate_upload_limits(&files)?;

        let existing = self
            .repository
            .find_by_game_id(game_id)
            .await?
            .unwrap_or_else(|| GameMedia::new_for_game(game_id));

        let (new_cover, new_screenshots, new_trailer) =
            self.process_and_upload_files(game_id, files).await?;

        let media = GameMedia::new(
            existing.id,
            game_id,
            new_cover.or(existing.cover),
            new_screenshots.unwrap_or(existing.screenshots),
            new_trailer.or(existing.trailer),
        );

        self.repository.upsert(media).await?;

        let saved = self.find_by_game_id(game_id).await?;
        self.to_response(saved).await
    }

    fn validate_upload_limits(&self, files: &[UploadedFile]) -> Result<()> {
        if files.is_empty() {
            return Err(MediaError::NoFilesProvided);
        }

        for file in files {
            let limit = if file.field_name == Trailer {
                MAX_VIDEO_BYTES
            } else {
                MAX_IMAGE_BYTES
            };
            if file.data.len() > limit {
                return Err(MediaError::FileTooLarge {
                    field: file.field_name.as_string(),
                    limit_mb: limit / 1024 / 1024,
                });
            }
        }
        Ok(())
    }

    async fn process_and_upload_files(
        &self,
        game_id: i32,
        files: Vec<UploadedFile>,
    ) -> Result<(Option<MediaFile>, Option<Vec<MediaFile>>, Option<MediaFile>)> {
        let now = DateTime::now();
        let mut cover = None;
        let mut trailer = None;
        let mut incoming_screenshots = vec![];
        let mut has_screenshots = false;
        let mut screenshot_index = 0;

        for file in files {
            let screenshot_seq = if file.field_name == Screenshot {
                screenshot_index += 1;
                Some(screenshot_index)
            } else {
                None
            };

            let object_key =
                Self::object_key(game_id, file.field_name, &file.file_name, screenshot_seq);
            let size_bytes = file.data.len();
            let mime_type = file.content_type.clone();

            self.upload_bytes(&object_key, file.content_type, file.data)
                .await?;

            match file.field_name {
                Cover => {
                    cover = Some(MediaFile::new(object_key, mime_type, size_bytes, now));
                }
                Screenshot => {
                    has_screenshots = true;
                    incoming_screenshots
                        .push(MediaFile::new(object_key, mime_type, size_bytes, now));
                }
                Trailer => {
                    trailer = Some(MediaFile::new(object_key, mime_type, size_bytes, now));
                }
            }
        }

        let screenshots = if has_screenshots {
            Some(incoming_screenshots)
        } else {
            None
        };
        Ok((cover, screenshots, trailer))
    }

    pub async fn delete_game_media(&self, game_id: i32) -> Result<()> {
        self.find_by_game_id(game_id).await?;

        self.minio
            .delete_object(&self.bucket, game_id.to_string())
            .send()
            .await
            .map_err(|e| MediaError::StorageError(e.to_string()))?;
        self.repository.delete_by_game_id(game_id).await
    }

    fn object_key(
        game_id: i32,
        field: FieldName,
        file_name: &str,
        screenshot_index: Option<usize>,
    ) -> String {
        let ext = file_name.rsplit('.').next().unwrap_or("bin");

        match field {
            Cover => format!("games/{game_id}/cover.{ext}"),
            Trailer => format!("games/{game_id}/trailer.{ext}"),
            Screenshot => {
                let seq = screenshot_index.unwrap_or_else(|| {
                    SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap_or_default()
                        .as_millis() as usize
                });
                format!("games/{game_id}/screenshot_{seq}.{ext}")
            }
        }
    }

    async fn upload_bytes(
        &self,
        object_key: &str,
        content_type: String,
        data: Bytes,
    ) -> Result<()> {
        let size = data.len();
        let segmented = SegmentedBytes::from(data);
        let mut extra_headers = Multimap::new();
        extra_headers.add("Content-Type", content_type);
        extra_headers.add("Content-Length", size.to_string());

        self.minio
            .put_object(&self.bucket, object_key, segmented)
            .extra_headers(Some(extra_headers))
            .send()
            .await
            .map_err(|e| MediaError::StorageError(e.to_string()))?;

        Ok(())
    }

    async fn find_by_game_id(&self, game_id: i32) -> Result<GameMedia> {
        self.repository
            .find_by_game_id(game_id)
            .await?
            .ok_or(MediaError::NotFound(game_id))
    }

    async fn presign(&self, object_key: &str) -> Result<String> {
        self.minio
            .get_presigned_object_url(&self.bucket, object_key, Method::GET)
            .expiry_seconds(60 * 60)
            .send()
            .await
            .map(|r| r.url)
            .map_err(|e| MediaError::StorageError(e.to_string()))
    }

    async fn media_file_to_response(&self, file: MediaFile) -> Result<MediaFileResponse> {
        let url = self.presign(&file.object_key).await?;
        Ok(MediaFileResponse::new(url, file.mime_type, file.size_bytes))
    }

    async fn to_response(&self, media: GameMedia) -> Result<GameMediaResponse> {
        let cover = match media.cover {
            Some(file) => Some(self.media_file_to_response(file).await?),
            None => None,
        };

        let mut screenshots = Vec::with_capacity(media.screenshots.len());
        for screenshot in media.screenshots {
            screenshots.push(self.media_file_to_response(screenshot).await?);
        }

        let trailer = match media.trailer {
            Some(file) => Some(self.media_file_to_response(file).await?),
            None => None,
        };

        Ok(GameMediaResponse::new(
            media.game_id,
            cover,
            screenshots,
            trailer,
        ))
    }
}
