use crate::dto::LibraryPagedResponse;
use crate::{
    error::{LibraryError, Result},
    model::{GameLibraryStatus, UserGame},
    repository::LibraryRepository,
};
use service_common::http_client::{CatalogueClient, CatalogueError};
use uuid::Uuid;

pub struct LibraryService {
    repository: Box<dyn LibraryRepository>,
    catalogue: CatalogueClient,
}

impl LibraryService {
    pub fn new(repository: Box<dyn LibraryRepository>, catalogue: CatalogueClient) -> Self {
        Self {
            repository,
            catalogue,
        }
    }

    pub async fn add_or_update_game(
        &self,
        user_id: Uuid,
        game_id: i32,
        status: GameLibraryStatus,
    ) -> Result<UserGame> {
        self.verify_game_exists(game_id).await?;
        self.repository.upsert_game(user_id, game_id, status).await
    }

    async fn verify_game_exists(&self, game_id: i32) -> Result<()> {
        self.catalogue
            .ensure_game_exists(game_id)
            .await
            .map_err(|error| match error {
                CatalogueError::NotFound(game_id) => LibraryError::InvalidGameId(game_id),
                CatalogueError::Unavailable(message) => {
                    LibraryError::CatalogueServiceError(message)
                }
            })
    }

    pub async fn get_user_library(
        &self,
        user_id: Uuid,
        status: Option<GameLibraryStatus>,
        page: u64,
        limit: u64,
    ) -> Result<LibraryPagedResponse> {
        self.repository
            .get_user_library(user_id, status, page, limit)
            .await
    }

    pub async fn remove_from_library(&self, user_id: Uuid, game_id: i32) -> Result<()> {
        self.repository.remove_game(user_id, game_id).await
    }
}
