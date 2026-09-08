use crate::{
    dto::{GameMediaResponse, MediaFileResponse},
    error::{MediaError, Result},
    media_keys,
    model::{FieldName::*, GameMedia, MediaFile, UploadedFile},
    repository::MediaRepository,
    storage::MediaStorage,
};
use mongodb::bson::DateTime;
use service_common::http_client::{CatalogueClient, CatalogueError};
use std::{collections::HashMap, sync::Arc};
use tracing::warn;

const MAX_IMAGE_BYTES: usize = 10 * 1024 * 1024;
const MAX_VIDEO_BYTES: usize = 500 * 1024 * 1024;

pub struct MediaService {
    repository: Box<dyn MediaRepository>,
    storage: Arc<dyn MediaStorage>,
    catalogue: CatalogueClient,
}

impl MediaService {
    pub fn new(
        repository: Box<dyn MediaRepository>,
        storage: Arc<dyn MediaStorage>,
        catalogue: CatalogueClient,
    ) -> Self {
        Self {
            repository,
            storage,
            catalogue,
        }
    }

    pub async fn get_game_media(&self, game_id: i32) -> Result<GameMediaResponse> {
        let media = self.find_by_game_id(game_id).await?;
        self.to_response(media).await
    }

    pub async fn get_game_covers_presigned_urls(
        &self,
        game_ids: &[i32],
    ) -> Result<HashMap<i32, Option<String>>> {
        let covers = self.repository.find_covers(game_ids).await?;
        let mut presigned = HashMap::with_capacity(covers.len());

        for (game_id, cover) in covers {
            let url = match cover {
                Some(cover) => Some(self.storage.presign(&cover.object_key).await?),
                None => None,
            };
            presigned.insert(game_id, url);
        }

        Ok(presigned)
    }

    pub async fn ensure_game_exists(&self, game_id: i32) -> Result<()> {
        self.catalogue
            .ensure_game_exists(game_id)
            .await
            .map_err(|error| match error {
                CatalogueError::NotFound(game_id) => MediaError::InvalidGameId(game_id),
                CatalogueError::Unavailable(message) => {
                    MediaError::CatalogueServiceError(message)
                }
            })
    }

    pub async fn upload_game_media(
        &self,
        game_id: i32,
        files: Vec<UploadedFile>,
        version: i64,
    ) -> Result<GameMediaResponse> {
        self.validate_upload_limits(&files)?;

        let existing = self
            .repository
            .find_by_game_id(game_id)
            .await?
            .unwrap_or_else(|| GameMedia::new_for_game(game_id));

        if existing.version != version {
            return Err(MediaError::Conflict(game_id));
        }

        let (new_cover, new_screenshots, new_trailer) =
            self.process_and_upload_files(game_id, files).await?;

        let uploaded_keys = media_keys::uploaded_keys(&new_cover, &new_screenshots, &new_trailer);
        let replaced_keys =
            media_keys::replaced_keys(&existing, &new_cover, &new_screenshots, &new_trailer);

        let media = GameMedia::new(
            existing.id,
            game_id,
            new_cover.or(existing.cover),
            new_screenshots.unwrap_or(existing.screenshots),
            new_trailer.or(existing.trailer),
            existing.version + 1,
        );

        if let Err(error) = self.repository.upsert(media, version).await {
            self.storage.delete_keys(&uploaded_keys).await;
            return Err(error);
        }

        self.storage.delete_keys(&replaced_keys).await;

        let saved = self.find_by_game_id(game_id).await?;
        self.to_response(saved).await
    }

    pub async fn delete_game_media(&self, game_id: i32, version: i64) -> Result<()> {
        let media = self.find_by_game_id(game_id).await?;
        if media.version != version {
            return Err(MediaError::Conflict(game_id));
        }
        let keys = media.object_keys();

        self.repository.delete_by_game_id(game_id, version).await?;
        self.storage.delete_keys(&keys).await;

        Ok(())
    }

    #[allow(dead_code)]
    async fn sweep_leftover_keys(&self, game_id: i32, known_keys: &[String]) {
        let mut known: Vec<String> = known_keys.to_vec();
        if let Ok(Some(current)) = self.repository.find_by_game_id(game_id).await {
            known.extend(current.object_keys());
        }

        match self
            .storage
            .list_prefix_keys(&media_keys::game_prefix(game_id))
            .await
        {
            Ok(listed_keys) => {
                let leftovers = media_keys::filter_unknown_keys(&known, listed_keys);
                self.storage.delete_keys(&leftovers).await;
            }
            Err(error) => {
                warn!(game_id, error = %error, "failed to list leftover media objects");
            }
        }
    }

    fn validate_upload_limits(&self, files: &[UploadedFile]) -> Result<()> {
        if files.is_empty() {
            return Err(MediaError::NoFilesProvided);
        }

        if files.len() > 22 {
            return Err(MediaError::TooManyFiles(String::from(
                "Too many files (max 22: 1 cover + 20 screenshots + 1 trailer)",
            )));
        }

        let screenshot_count = files
            .iter()
            .filter(|file| file.field_name == Screenshot)
            .count();
        if screenshot_count > 20 {
            return Err(MediaError::TooManyFiles(String::from(
                "Too many screenshots (max 20)",
            )));
        }

        let total_bytes: usize = files.iter().map(|file| file.data.len()).sum();
        if total_bytes > MAX_VIDEO_BYTES + 20 * MAX_IMAGE_BYTES + MAX_IMAGE_BYTES {
            return Err(MediaError::TooManyFiles(String::from(
                "Total upload size exceeds the allowed limit",
            )));
        }

        for file in files {
            let limit = if file.field_name == Trailer {
                MAX_VIDEO_BYTES
            } else {
                MAX_IMAGE_BYTES
            };
            if file.data.is_empty() {
                return Err(MediaError::NoFilesProvided);
            }
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
        let attempt = media_keys::unique_attempt_suffix();
        let mut cover = None;
        let mut trailer = None;
        let mut incoming_screenshots = vec![];
        let mut has_screenshots = false;
        let mut screenshot_index = 0;
        let mut uploaded_keys: Vec<String> = Vec::new();

        for file in files {
            let screenshot_seq = if file.field_name == Screenshot {
                screenshot_index += 1;
                Some(screenshot_index)
            } else {
                None
            };

            let object_key = media_keys::staged_object_key(
                game_id,
                file.field_name,
                &file.file_name,
                screenshot_seq,
                &attempt,
            );
            let size_bytes = file.data.len();
            let mime_type = file.content_type.clone();

            if let Err(error) = self
                .storage
                .put_object(&object_key, file.content_type, file.data)
                .await
            {
                self.storage.delete_keys(&uploaded_keys).await;
                return Err(error);
            }
            uploaded_keys.push(object_key.clone());

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

    async fn find_by_game_id(&self, game_id: i32) -> Result<GameMedia> {
        self.repository
            .find_by_game_id(game_id)
            .await?
            .ok_or(MediaError::NotFound(game_id))
    }

    async fn media_file_to_response(&self, file: MediaFile) -> Result<MediaFileResponse> {
        let url = self.storage.presign(&file.object_key).await?;
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
            media.version,
        ))
    }
}
