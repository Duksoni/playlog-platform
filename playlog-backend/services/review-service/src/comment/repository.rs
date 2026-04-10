use super::{Comment, CommentError, CommentTargetType, Result, SimpleCommentResponse};
use async_trait::async_trait;
use bson::{Binary, DateTime};
use futures::StreamExt;
use mongodb::{
    bson::{doc, oid::ObjectId},
    Collection,
};
use uuid::Uuid;

const PAGE_SIZE: i64 = 10;

#[async_trait]
pub trait CommentRepository: Send + Sync {
    async fn find_comments_by_target(
        &self,
        target_type: CommentTargetType,
        target_id: &str,
        page: u64,
    ) -> Result<Vec<SimpleCommentResponse>>;
    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Comment>>;
    async fn find_one_by_user_id(&self, id: ObjectId, user_id: Uuid) -> Result<Option<Comment>>;
    async fn upsert(&self, comment: Comment) -> Result<Comment>;
    async fn delete(&self, id: ObjectId, version: i64) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct MongoCommentRepository {
    comments: Collection<Comment>,
}

impl MongoCommentRepository {
    pub fn new(comments: Collection<Comment>) -> Self {
        Self { comments }
    }
}

#[async_trait]
impl CommentRepository for MongoCommentRepository {
    async fn find_comments_by_target(
        &self,
        target_type: CommentTargetType,
        target_id: &str,
        page: u64,
    ) -> Result<Vec<SimpleCommentResponse>> {
        let skip = (page.max(1) - 1) * PAGE_SIZE as u64;
        let filter = doc! {
            "target_type": target_type.as_db_value(),
            "target_id": target_id,
            "deleted": false
        };
        let mut cursor = self
            .comments
            .find(filter)
            .sort(doc! { "created_at": -1 })
            .limit(PAGE_SIZE)
            .skip(skip)
            .await?;
        let mut comments = vec![];
        while let Some(comment) = cursor.next().await {
            comments.push(comment?.into());
        }
        Ok(comments)
    }

    async fn find_by_id(&self, id: ObjectId) -> Result<Option<Comment>> {
        let filter = doc! { "_id": id, "deleted": false };
        Ok(self.comments.find_one(filter).await?)
    }

    async fn find_one_by_user_id(&self, id: ObjectId, user_id: Uuid) -> Result<Option<Comment>> {
        let uuid_bytes = user_id.as_bytes().to_vec();
        let binary = Binary {
            subtype: bson::spec::BinarySubtype::Generic,
            bytes: uuid_bytes,
        };
        let filter = doc! {
            "_id": id,
            "user_id": binary,
            "deleted": false,
        };
        Ok(self.comments.find_one(filter).await?)
    }

    async fn upsert(&self, mut comment: Comment) -> Result<Comment> {
        match comment.id {
            Some(id) => {
                let current_version = comment.version;
                comment.version += 1;
                let filter = doc! { "_id": id, "version": current_version };
                let result = self.comments.replace_one(filter, comment.clone()).await?;
                if result.matched_count == 0 {
                    return Err(CommentError::Conflict(id));
                }
                Ok(comment)
            }
            None => {
                let result = self.comments.insert_one(comment.clone()).await?;
                comment.id = Some(result.inserted_id.as_object_id().unwrap());
                Ok(comment)
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
        let result = self.comments.update_one(filter, update).await?;
        if result.matched_count == 0 {
            return Err(CommentError::Conflict(id));
        }
        Ok(())
    }
}
