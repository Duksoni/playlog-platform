use super::{
    Comment, CommentError, CommentRepository, CommentTargetType, CreateCommentRequest,
    DetailedCommentResponse, RecentGameCommentResponse, Result, SimpleCommentResponse,
    UpdateCommentRequest,
};
use crate::{review::ReviewRepository, shared::ensure_game_exists};
use bson::DateTime;
use mongodb::bson::oid::ObjectId;
use reqwest::Client as HttpClient;
use uuid::Uuid;

pub struct CommentService {
    comment_repository: Box<dyn CommentRepository>,
    review_repository: Box<dyn ReviewRepository>,
    http_client: HttpClient,
    catalogue_service_url: String,
}

impl CommentService {
    pub fn new(
        comment_repository: Box<dyn CommentRepository>,
        review_repository: Box<dyn ReviewRepository>,
        http_client: HttpClient,
        catalogue_service_url: String,
    ) -> Self {
        Self {
            comment_repository,
            review_repository,
            http_client,
            catalogue_service_url,
        }
    }

    pub async fn get(&self, id: ObjectId) -> Result<DetailedCommentResponse> {
        self.comment_repository
            .find_by_id(id)
            .await?
            .map(Comment::into)
            .ok_or(CommentError::NotFound)
    }

    pub async fn get_recent_game_comments(
        &self,
        limit: u64,
    ) -> Result<Vec<RecentGameCommentResponse>> {
        self.comment_repository
            .find_recent_game_comments(limit)
            .await
    }

    pub async fn get_for_target(
        &self,
        target_type: CommentTargetType,
        target_id: &str,
        page: u64,
    ) -> Result<Vec<SimpleCommentResponse>> {
        self.comment_repository
            .find_comments_by_target(target_type, target_id, page)
            .await
    }

    pub async fn get_one_for_user(
        &self,
        user_id: Uuid,
        id: ObjectId,
    ) -> Result<DetailedCommentResponse> {
        self.requre_get_for_user(id, user_id)
            .await
            .map(Comment::into)
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        username: String,
        request: CreateCommentRequest,
    ) -> Result<DetailedCommentResponse> {
        match request.target_type {
            CommentTargetType::Game => {
                let game_id = request
                    .target_id
                    .parse::<i32>()
                    .map_err(|_| CommentError::InvalidGameId(request.target_id.clone()))?;
                ensure_game_exists(&self.http_client, &self.catalogue_service_url, game_id).await?;
            }
            CommentTargetType::Review => {
                let review_id = ObjectId::parse_str(&request.target_id)
                    .map_err(|_| CommentError::InvalidReviewId(request.target_id.clone()))?;
                self.review_repository
                    .find_by_id(review_id)
                    .await
                    .map_err(|_| CommentError::InvalidReviewId(review_id.to_string()))?;
            }
        }

        let comment = Comment::new(
            request.target_type,
            request.target_id,
            user_id,
            username,
            request.text,
            DateTime::now(),
        );
        self.comment_repository
            .upsert(comment)
            .await
            .map(Comment::into)
    }

    pub async fn update_text(
        &self,
        user_id: Uuid,
        id: ObjectId,
        request: UpdateCommentRequest,
    ) -> Result<DetailedCommentResponse> {
        let mut comment = self.requre_get_for_user(id, user_id).await?;
        comment.text = request.text;
        comment.updated_at = DateTime::now();
        self.comment_repository
            .upsert(comment)
            .await
            .map(Comment::into)
    }

    pub async fn delete(&self, user_id: Uuid, id: ObjectId) -> Result<()> {
        let comment = self.requre_get_for_user(id, user_id).await?;
        if comment.user_id != user_id {
            return Err(CommentError::Unauthorized);
        }
        self.comment_repository.delete(id, comment.version).await
    }

    async fn requre_get_for_user(&self, id: ObjectId, user_id: Uuid) -> Result<Comment> {
        self.comment_repository
            .find_one_by_user_id(id, user_id)
            .await?
            .ok_or(CommentError::NotFound)
    }
}
