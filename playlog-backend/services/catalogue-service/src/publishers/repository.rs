use async_trait::async_trait;
use sqlx::{query_as, PgPool};

use super::{Publisher, Result};

#[async_trait]
pub trait PublisherRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Publisher>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Publisher>>;
    async fn search_by_name(&self, query: &str) -> Result<Vec<Publisher>>;
    async fn create(&self, name: &str) -> Result<Publisher>;
    async fn update_name(&self, id: i32, name: &str) -> Result<Publisher>;
}

#[derive(Debug, Clone)]
pub struct PostgresPublisherRepository {
    pool: PgPool,
}

impl PostgresPublisherRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PublisherRepository for PostgresPublisherRepository {
    async fn find_all(&self) -> Result<Vec<Publisher>> {
        let publishers = query_as!(
            Publisher,
            "SELECT id, name FROM publishers ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(publishers)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Publisher>> {
        let publisher = query_as!(
            Publisher,
            "SELECT id, name FROM publishers WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(publisher)
    }

    async fn search_by_name(&self, query: &str) -> Result<Vec<Publisher>> {
        let publishers = query_as!(
            Publisher,
            "SELECT id, name FROM publishers WHERE name ILIKE $1 ORDER BY name LIMIT 20",
            format!("%{}%", query)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(publishers)
    }

    async fn create(&self, name: &str) -> Result<Publisher> {
        let publisher = query_as!(
            Publisher,
            "INSERT INTO publishers (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(publisher)
    }

    async fn update_name(&self, id: i32, name: &str) -> Result<Publisher> {
        let publisher = query_as!(
            Publisher,
            "UPDATE publishers SET name = $1 WHERE id = $2 RETURNING id, name",
            name,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(super::error::PublisherError::NotFound(id))?;

        Ok(publisher)
    }
}
