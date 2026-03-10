use crate::{
    error::{MediaError, Result},
    model::GameMedia,
};
use async_trait::async_trait;
use mongodb::{bson::doc, options::ReplaceOptions, Collection};

#[async_trait]
pub trait MediaRepository: Send + Sync {
    async fn find_by_game_id(&self, game_id: i32) -> Result<Option<GameMedia>>;
    async fn upsert(&self, media: GameMedia) -> Result<()>;
    async fn delete_by_game_id(&self, game_id: i32) -> Result<()>;
}

pub struct MongoMediaRepository {
    collection: Collection<GameMedia>,
}

impl MongoMediaRepository {
    pub fn new(collection: Collection<GameMedia>) -> Self {
        Self { collection }
    }
}

#[async_trait]
impl MediaRepository for MongoMediaRepository {
    async fn find_by_game_id(&self, game_id: i32) -> Result<Option<GameMedia>> {
        self.collection
            .find_one(doc! { "game_id": game_id })
            .await
            .map_err(|e| MediaError::DatabaseError(e.to_string()))
    }

    async fn upsert(&self, media: GameMedia) -> Result<()> {
        let filter = doc! { "game_id": media.game_id };
        let options = ReplaceOptions::builder().upsert(true).build();

        self.collection
            .replace_one(filter, media)
            .with_options(options)
            .await
            .map_err(|e| MediaError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    async fn delete_by_game_id(&self, game_id: i32) -> Result<()> {
        self.collection
            .delete_one(doc! { "game_id": game_id })
            .await
            .map_err(|e| MediaError::DatabaseError(e.to_string()))?;

        Ok(())
    }
}
