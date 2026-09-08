use super::{
    CreateUpdateReviewRequest, GameRatingStatsResponse, GameReviewResponse,
    MostReviewedGameResponse, Rating, RecentReviewResponse, Result, Review, ReviewDetailedResponse,
    ReviewError, ReviewRepository, ReviewSimpleResponse, TopGameResponse,
};
use bson::DateTime;
use mongodb::bson::oid::ObjectId;
use service_common::http_client::{CatalogueClient, CatalogueError};
use uuid::Uuid;

pub struct ReviewService {
    repository: Box<dyn ReviewRepository>,
    catalogue: CatalogueClient,
}

impl ReviewService {
    pub fn new(repository: Box<dyn ReviewRepository>, catalogue: CatalogueClient) -> Self {
        Self {
            repository,
            catalogue,
        }
    }

    pub async fn get(&self, id: ObjectId) -> Result<ReviewDetailedResponse> {
        self.repository
            .find_by_id(id)
            .await?
            .map(Review::into)
            .ok_or(ReviewError::NotFound)
    }

    pub async fn get_recent(&self, limit: u64) -> Result<Vec<RecentReviewResponse>> {
        self.repository.find_recent(limit).await
    }

    pub async fn get_top_rated_games(&self, limit: u64) -> Result<Vec<TopGameResponse>> {
        self.repository.find_top_rated_games(limit).await
    }

    pub async fn get_most_reviewed_games(
        &self,
        limit: u64,
    ) -> Result<Vec<MostReviewedGameResponse>> {
        self.repository.find_most_reviewed_games(limit).await
    }

    pub async fn get_for_game(
        &self,
        game_id: i32,
        rating: Option<Rating>,
        page: u64,
    ) -> Result<Vec<GameReviewResponse>> {
        self.repository.find_by_game(game_id, rating, page).await
    }

    pub async fn get_rating_stats_for_game(&self, game_id: i32) -> Result<GameRatingStatsResponse> {
        self.repository
            .find_stats_for_game(game_id)
            .await?
            .ok_or(ReviewError::NotFound)
    }

    pub async fn get_for_user_and_game(
        &self,
        user_id: Uuid,
        game_id: i32,
    ) -> Result<ReviewSimpleResponse> {
        self.repository
            .find_by_user_and_game(user_id, game_id)
            .await?
            .map(Review::into)
            .ok_or(ReviewError::NotFound)
    }

    pub async fn upsert(
        &self,
        user_id: Uuid,
        username: String,
        request: CreateUpdateReviewRequest,
    ) -> Result<ReviewDetailedResponse> {
        match self.catalogue.ensure_game_exists(request.game_id).await {
            Ok(()) => {}
            Err(CatalogueError::NotFound(game_id)) => {
                return Err(ReviewError::InvalidGameId(game_id));
            }
            Err(CatalogueError::Unavailable(message)) => {
                return Err(ReviewError::CatalogueServiceError(message));
            }
        }

        let now = DateTime::now();
        let existing = self
            .repository
            .find_by_user_and_game(user_id, request.game_id)
            .await?;

        let review = match existing {
            Some(mut review) => {
                review.rating = request.rating;
                review.text = request.text;
                review.updated_at = now;
                review
            }
            None => Review::new(
                request.game_id,
                user_id,
                username,
                request.rating,
                request.text,
                now,
            ),
        };

        self.repository.upsert(review).await.map(Review::into)
    }

    pub async fn delete(&self, user_id: Uuid, id: ObjectId) -> Result<()> {
        let review = self
            .repository
            .find_by_id(id)
            .await?
            .ok_or(ReviewError::NotFound)?;
        if review.user_id != user_id {
            return Err(ReviewError::Unauthorized);
        }
        self.repository.delete(id, review.version).await
    }
}
