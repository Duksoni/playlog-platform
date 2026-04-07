use super::{
    CreateGameRequest, Game, GameDetails, GameError, GameFilterQuery, GameRepository, GameSimple,
    Result, UpdateGameRequest,
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

    pub async fn get(&self, id: i32) -> Result<Game> {
        self.game_repository
            .get(id, true)
            .await?
            .ok_or(GameError::NotFound(id))
    }

    pub async fn get_details(&self, id: i32, include_draft: bool) -> Result<GameDetails> {
        self.game_repository
            .get_details(id, include_draft)
            .await?
            .ok_or(GameError::NotFound(id))
    }

    pub async fn create(&self, request: CreateGameRequest) -> Result<Game> {
        validate_entity_ids(
            Some(&request.developer_ids),
            "developers",
            &*self.developer_repository,
        )
        .await?;
        validate_entity_ids(
            Some(&request.publisher_ids),
            "publishers",
            &*self.publisher_repository,
        )
        .await?;
        validate_entity_ids(
            Some(&request.platform_ids),
            "platforms",
            &*self.platform_repository,
        )
        .await?;
        validate_entity_ids(Some(&request.genre_ids), "genres", &*self.genre_repository).await?;
        validate_entity_ids(Some(&request.tag_ids), "tags", &*self.tag_repository).await?;

        self.game_repository.create(request).await
    }

    pub async fn update(&self, id: i32, request: UpdateGameRequest) -> Result<GameDetails> {
        validate_entity_ids(
            request.developer_ids.as_deref(),
            "developers",
            &*self.developer_repository,
        )
        .await?;
        validate_entity_ids(
            request.publisher_ids.as_deref(),
            "publishers",
            &*self.publisher_repository,
        )
        .await?;
        validate_entity_ids(
            request.platform_ids.as_deref(),
            "platforms",
            &*self.platform_repository,
        )
        .await?;
        validate_entity_ids(
            request.genre_ids.as_deref(),
            "genres",
            &*self.genre_repository,
        )
        .await?;
        validate_entity_ids(request.tag_ids.as_deref(), "tags", &*self.tag_repository).await?;

        self.game_repository.update(id, request).await
    }

    pub async fn delete(&self, id: i32) -> Result<()> {
        self.game_repository.delete(id).await
    }

    pub async fn publish(&self, id: i32, version: i64) -> Result<Game> {
        self.change_draft(id, false, version).await
    }

    pub async fn unpublish(&self, id: i32, version: i64) -> Result<Game> {
        self.change_draft(id, true, version).await
    }

    async fn change_draft(&self, id: i32, draft: bool, version: i64) -> Result<Game> {
        self.game_repository.set_draft(id, draft, version).await
    }
}

async fn validate_entity_ids(
    entity_ids: Option<&[i32]>,
    entity_type: &str,
    repository: &dyn GameEntityRepository,
) -> Result<()> {
    let Some(ids) = entity_ids else {
        return Ok(());
    };

    if ids.is_empty() {
        return Err(GameError::NoIdsProvided(String::from(entity_type)));
    }

    for id in ids {
        repository.exists(*id).await?;
    }

    Ok(())
}
