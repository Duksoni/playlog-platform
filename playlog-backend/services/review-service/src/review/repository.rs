use super::{
    GameRatingStatsResponse, GameReviewResponse, MostReviewedGameResponse, Rating,
    RecentReviewResponse, Result, Review, ReviewError, TopGameResponse,
};
use anyhow::anyhow;
use async_trait::async_trait;
use bson::{serialize_to_bson, Binary, DateTime, Document};
use futures::StreamExt;
use mongodb::{
    bson, bson::{doc, oid::ObjectId},
    Collection,
};
use std::str::FromStr;
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
    async fn find_recent(&self, limit: u64) -> Result<Vec<RecentReviewResponse>>;
    async fn find_top_rated_games(&self, limit: u64) -> Result<Vec<TopGameResponse>>;
    async fn find_most_reviewed_games(&self, limit: u64) -> Result<Vec<MostReviewedGameResponse>>;
    async fn find_stats_for_game(&self, game_id: i32) -> Result<Option<GameRatingStatsResponse>>;
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

    async fn find_recent(&self, limit: u64) -> Result<Vec<RecentReviewResponse>> {
        let filter = doc! {
            "deleted": false
        };
        let mut cursor = self
            .reviews
            .find(filter)
            .sort(doc! { "created_at": -1 })
            .limit(limit as i64)
            .await?;
        let mut reviews = vec![];
        while let Some(review) = cursor.next().await {
            reviews.push(review?.into());
        }
        Ok(reviews)
    }

    async fn find_top_rated_games(&self, limit: u64) -> Result<Vec<TopGameResponse>> {
        let pipeline = Self::make_top_rated_games_pipeline(limit);

        let mut cursor = self.reviews.aggregate(pipeline).await?;
        let mut result = vec![];

        while let Some(item) = cursor.next().await {
            let doc = item?;
            let game_id = doc.get_i32("_id").map_err(|e| anyhow!(e))?;
            let average_rating = doc.get_f64("averageRating").map_err(|e| anyhow!(e))?;

            result.push(TopGameResponse::new(game_id, average_rating));
        }

        Ok(result)
    }

    async fn find_most_reviewed_games(&self, limit: u64) -> Result<Vec<MostReviewedGameResponse>> {
        let pipeline = Self::make_most_reviewed_games_pipeline(limit);
        let mut cursor = self.reviews.aggregate(pipeline).await?;
        let mut result = vec![];

        while let Some(item) = cursor.next().await {
            let doc = item?;
            let game_id = doc.get_i32("_id").map_err(|e| anyhow!(e))?;
            let review_count = doc.get_i32("reviewCount").map_err(|e| anyhow!(e))?;

            result.push(MostReviewedGameResponse::new(game_id, review_count));
        }

        Ok(result)
    }

    async fn find_stats_for_game(&self, game_id: i32) -> Result<Option<GameRatingStatsResponse>> {
        let pipeline = vec![
            doc! {
                "$match": {
                    "game_id": game_id,
                    "deleted": false,
                }
            },
            doc! {
                "$group": {
                    "_id": "$rating",
                    "count": { "$sum": 1 }
                }
            },
        ];

        let mut cursor = self.reviews.aggregate(pipeline).await?;

        let mut highly_recommended_count = 0_i64;
        let mut good_count = 0_i64;
        let mut okay_count = 0_i64;
        let mut not_recommended_count = 0_i64;

        while let Some(result) = cursor.next().await {
            let doc = result?;
            let rating = doc.get_str("_id").map_err(|e| anyhow!(e))?;
            let rating = Rating::from_str(rating).map_err(|e| anyhow!(e))?;
            let count = doc.get_i32("count").map_err(|e| anyhow!(e))? as i64;

            match rating {
                Rating::HighlyRecommended => highly_recommended_count = count,
                Rating::Good => good_count = count,
                Rating::Okay => okay_count = count,
                Rating::NotRecommended => not_recommended_count = count,
            }
        }

        Ok(Some(GameRatingStatsResponse::new(
            highly_recommended_count,
            good_count,
            okay_count,
            not_recommended_count,
        )))
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

impl MongoReviewRepository {
    fn make_top_rated_games_pipeline(limit: u64) -> Vec<Document> {
        vec![
            doc! {
                "$match": {
                    "deleted": false
                }
            },
            doc! {
                "$addFields": {
                    "ratingScore": {
                        "$switch": {
                            "branches": [
                                {
                                    "case": { "$eq": ["$rating", Rating::NotRecommended.as_db_value()] }, "then": 1 },
                                {
                                    "case": { "$eq": ["$rating", Rating::Okay.as_db_value()] }, "then": 2 },
                                {
                                    "case": { "$eq": ["$rating",Rating::Good.as_db_value()] }, "then": 3 },
                                {
                                    "case": { "$eq": ["$rating", Rating::HighlyRecommended.as_db_value()] }, "then": 4 },
                            ],
                            "default": 0
                        }
                    }
                }
            },
            doc! {
                "$group": {
                    "_id": "$game_id",
                    "averageRating": { "$avg": "$ratingScore" }
                }
            },
            doc! {
                "$sort": {
                    "averageRating": -1
                }
            },
            doc! {
                "$limit": limit as i64
            },
        ]
    }

    fn make_most_reviewed_games_pipeline(limit: u64) -> Vec<Document> {
        vec![
            doc! {
                "$match": {
                    "deleted": false
                }
            },
            doc! {
                "$group": {
                    "_id": "$game_id",
                    "reviewCount": { "$sum": 1 }
                }
            },
            doc! {
                "$sort": {
                    "reviewCount": -1
                }
            },
            doc! {
                "$limit": limit as i64
            },
        ]
    }
}
