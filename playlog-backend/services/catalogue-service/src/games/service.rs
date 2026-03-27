use super::{
    CreateUpdateGameRequest, Game, GameDetails, GameError, GameFilterQuery, GameRepository,
    GameSimple, Result,
};
use crate::entity::GameEntityRepository;

pub struct GameService {
    game_repository: Box<dyn GameRepository>,
    developer_repository: Box<dyn GameEntityRepository>,
    publisher_repository: Box<dyn GameEntityRepository>,
    platform_repository: Box<dyn GameEntityRepository>,
    genre_repository: Box<dyn GameEntityRepository>,
    tag_repository: Box<dyn GameEntityRepository>,
}

impl GameService {
    pub fn new(
        game_repository: Box<dyn GameRepository>,
        developer_repository: Box<dyn GameEntityRepository>,
        publisher_repository: Box<dyn GameEntityRepository>,
        platform_repository: Box<dyn GameEntityRepository>,
        genre_repository: Box<dyn GameEntityRepository>,
        tag_repository: Box<dyn GameEntityRepository>,
    ) -> Self {
        Self {
            game_repository,
            developer_repository,
            publisher_repository,
            platform_repository,
            genre_repository,
            tag_repository,
        }
    }

    pub async fn filter(
        &self,
        include_drafts: bool,
        params: GameFilterQuery,
    ) -> Result<Vec<GameSimple>> {
        self.game_repository.filter(include_drafts, params).await
    }

    pub async fn find_by_developer(&self, developer_id: i32) -> Result<Vec<GameSimple>> {
        self.game_repository.find_by_developer(developer_id).await
    }

    pub async fn find_by_publisher(&self, publisher_id: i32, page: u64) -> Result<Vec<GameSimple>> {
        self.game_repository
            .find_by_publisher(publisher_id, page)
            .await
    }

    pub async fn get_all_unpublished(&self) -> Result<Vec<GameSimple>> {
        self.game_repository.get_all_unpublished().await
    }

    pub async fn get(&self, id: i32, include_draft: bool) -> Result<Game> {
        self.game_repository
            .get(id, include_draft)
            .await?
            .ok_or(GameError::NotFound(id))
    }

    pub async fn get_details(&self, id: i32, include_draft: bool) -> Result<GameDetails> {
        self.game_repository
            .get_details(id, include_draft)
            .await?
            .ok_or(GameError::NotFound(id))
    }

    pub async fn create(&self, request: CreateUpdateGameRequest) -> Result<Game> {
        self.ensure_entities_exist(&request).await?;
        self.game_repository.create(request).await
    }

    pub async fn update(&self, id: i32, request: CreateUpdateGameRequest) -> Result<GameDetails> {
        self.ensure_entities_exist(&request).await?;
        self.game_repository.update(id, request).await
    }

    pub async fn delete(&self, id: i32) -> Result<()> {
        if let Some(_game) = self.game_repository.get(id, true).await? {
            self.game_repository.delete(id).await
        } else {
            Err(GameError::NotFound(id))
        }
    }

    pub async fn publish(&self, id: i32) -> Result<Game> {
        if let Some(_game) = self.game_repository.get(id, true).await? {
            self.game_repository.set_draft(id, false).await
        } else {
            Err(GameError::NotFound(id))
        }
    }

    pub async fn unpublish(&self, id: i32) -> Result<Game> {
        if let Some(_game) = self.game_repository.get(id, false).await? {
            self.game_repository.set_draft(id, true).await
        } else {
            Err(GameError::NotFound(id))
        }
    }

    async fn ensure_entities_exist(&self, request: &CreateUpdateGameRequest) -> Result<()> {
        for developer_id in &request.developer_ids {
            self.developer_repository.exists(*developer_id).await?;
        }
        for publisher_id in &request.publisher_ids {
            self.publisher_repository.exists(*publisher_id).await?;
        }
        for platform_id in &request.platform_ids {
            self.platform_repository.exists(*platform_id).await?;
        }
        for genre_id in &request.genre_ids {
            self.genre_repository.exists(*genre_id).await?;
        }
        for tag_id in &request.tag_ids {
            self.tag_repository.exists(*tag_id).await?;
        }
        Ok(())
    }
}
