use super::{GameReviewResponse, Rating, Result, Review, ReviewError};
use anyhow::anyhow;
use async_trait::async_trait;
use bson::{serialize_to_bson, Binary, DateTime};
use futures::StreamExt;
use mongodb::{
    bson, bson::{doc, oid::ObjectId},
    Collection,
};
use uuid::Uuid;

const PAGE_SIZE: i64 = 10;

#[async_trait]
pub trait ReviewRepository: Send + Sync {
    async fn find_by_game(
        &self,
        game_id: i32,
        rating: Option<Rating>,
        page: u64,
    ) -> Result<Vec<GameReviewResponse>>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Review>>;
    async fn find_by_user_and_game(&self, user_id: Uuid, game_id: i32) -> Result<Option<Review>>;
    async fn upsert(&self, review: Review) -> Result<Review>;
    async fn delete(&self, id: ObjectId, version: i64) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct MongoReviewRepository {
    reviews: Collection<Review>,
}

impl MongoReviewRepository {
    pub fn new(reviews: Collection<Review>) -> Self {
        Self { reviews }
    }
}

#[async_trait]
impl ReviewRepository for MongoReviewRepository {
    async fn find_by_game(
        &self,
        game_id: i32,
        rating: Option<Rating>,
        page: u64,
    ) -> Result<Vec<GameReviewResponse>> {
        let skip = (page.max(1) - 1) * PAGE_SIZE as u64;

        let filter = if let Some(rating) = rating {
            let rating_bson = serialize_to_bson(&rating).map_err(|e| anyhow!(e))?;
            doc! {
                "game_id": game_id,
                "rating": rating_bson,
                "deleted": false,
            }
        } else {
            doc! { "game_id": game_id, "deleted": false }
        };
        let mut reviews = vec![];
        let mut cursor = self
            .reviews
            .find(filter)
            .sort(doc! { "created_at": -1 })
            .limit(PAGE_SIZE)
            .skip(skip)
            .await?;
        while let Some(review) = cursor.next().await {
            reviews.push(review?.into());
        }
        Ok(reviews)
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Review>> {
        let review = self
            .reviews
            .find_one(doc! { "_id": id, "deleted": false })
            .await?;
        Ok(review)
    }

    async fn find_by_user_and_game(&self, user_id: Uuid, game_id: i32) -> Result<Option<Review>> {
        let uuid_bytes = user_id.as_bytes().to_vec();
        let binary = Binary {
            subtype: bson::spec::BinarySubtype::Generic,
            bytes: uuid_bytes,
        };
        let filter = doc! {
            "user_id": binary,
            "game_id": game_id,
            "deleted": false,
        };
        let review = self.reviews.find_one(filter).await?;
        Ok(review)
    }

    async fn upsert(&self, mut review: Review) -> Result<Review> {
        match review.id {
            Some(id) => {
                let current_version = review.version;
                review.version += 1;
                let filter = doc! { "_id": id, "version": current_version };
                let result = self.reviews.replace_one(filter, review.clone()).await?;
                if result.matched_count == 0 {
                    return Err(ReviewError::Conflict(id));
                }
                Ok(review)
            }
            None => {
                let result = self.reviews.insert_one(review.clone()).await?;
                review.id = Some(result.inserted_id.as_object_id().unwrap());
                Ok(review)
            }
        }
    }

    async fn delete(&self, id: ObjectId, version: i64) -> Result<()> {
        let filter = doc! { "_id": id, "version": version };
        let update = doc! {
            "$set": {
                "deleted": true,
                "updated_at": DateTime::now(),
            },
            "$inc": { "version": 1 }
        };
        let result = self.reviews.update_one(filter, update).await?;
        if result.matched_count == 0 {
            return Err(ReviewError::Conflict(id));
        }
        Ok(())
    }
}
