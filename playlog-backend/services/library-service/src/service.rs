use crate::model::LibraryGame;
use crate::{
    error::{LibraryError, Result},
    model::{GameLibraryStatus, UserGame},
    repository::LibraryRepository,
};
use reqwest::Client;
use uuid::Uuid;

pub struct LibraryService {
    repository: Box<dyn LibraryRepository>,
    client: Client,
    catalogue_url: String,
}

impl LibraryService {
    pub fn new(
        repository: Box<dyn LibraryRepository>,
        client: Client,
        catalogue_url: String,
    ) -> Self {
        Self {
            repository,
            client,
            catalogue_url,
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
        let url = format!("{}/api/games/{}", self.catalogue_url, game_id);

        let response = self
            .client
            .get(url)
            .send()
            .await
            .map_err(|e| LibraryError::CatalogueServiceError(e.to_string()))?;

        match response.status() {
            reqwest::StatusCode::OK => Ok(()),
            reqwest::StatusCode::NOT_FOUND => Err(LibraryError::InvalidGameId(game_id)),
            _ => Err(LibraryError::CatalogueServiceError(format!(
                "Unexpected status: {}",
                response.status()
            ))),
        }
    }

    pub async fn get_user_library(
        &self,
        user_id: Uuid,
        status: Option<GameLibraryStatus>,
    ) -> Result<Vec<LibraryGame>> {
        self.repository.get_user_library(user_id, status).await
    }

    pub async fn remove_from_library(&self, user_id: Uuid, game_id: i32) -> Result<()> {
        self.repository.remove_game(user_id, game_id).await
    }
}
