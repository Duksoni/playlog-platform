use async_trait::async_trait;
use sqlx::{query_as, PgPool};

use super::{Result, Tag};

#[async_trait]
pub trait TagRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Tag>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Tag>>;
    async fn create(&self, name: &str) -> Result<Tag>;
}

#[derive(Debug, Clone)]
pub struct PostgresTagRepository {
    pool: PgPool,
}

impl PostgresTagRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl TagRepository for PostgresTagRepository {
    async fn find_all(&self) -> Result<Vec<Tag>> {
        let tags = query_as!(Tag, "SELECT id, name FROM tags ORDER BY name")
            .fetch_all(&self.pool)
            .await?;

        Ok(tags)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Tag>> {
        let tag = query_as!(Tag, "SELECT id, name FROM tags WHERE id = $1", id)
            .fetch_optional(&self.pool)
            .await?;

        Ok(tag)
    }

    async fn create(&self, name: &str) -> Result<Tag> {
        let tag = query_as!(
            Tag,
            "INSERT INTO tags (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(tag)
    }
}
