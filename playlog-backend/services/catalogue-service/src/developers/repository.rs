use async_trait::async_trait;
use sqlx::{query_as, PgPool};

use super::{Developer, error::Result};

#[async_trait]
pub trait DeveloperRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Developer>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Developer>>;
    async fn search_by_name(&self, query: &str) -> Result<Vec<Developer>>;
    async fn create(&self, name: &str) -> Result<Developer>;
    async fn update_name(&self, id: i32, name: &str) -> Result<Developer>;
}

#[derive(Debug, Clone)]
pub struct PostgresDeveloperRepository {
    pool: PgPool,
}

impl PostgresDeveloperRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DeveloperRepository for PostgresDeveloperRepository {
    async fn find_all(&self) -> Result<Vec<Developer>> {
        let developers = query_as!(
            Developer,
            "SELECT id, name FROM developers ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(developers)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Developer>> {
        let developer = query_as!(
            Developer,
            "SELECT id, name FROM developers WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(developer)
    }

    async fn search_by_name(&self, query: &str) -> Result<Vec<Developer>> {
        let developers = query_as!(
            Developer,
            "SELECT id, name FROM developers WHERE name ILIKE $1 ORDER BY name LIMIT 20",
            format!("%{}%", query)
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(developers)
    }

    async fn create(&self, name: &str) -> Result<Developer> {
        let developer = query_as!(
            Developer,
            "INSERT INTO developers (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(developer)
    }

    async fn update_name(&self, id: i32, name: &str) -> Result<Developer> {
        let developer = query_as!(
            Developer,
            "UPDATE developers SET name = $1 WHERE id = $2 RETURNING id, name",
            name,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(super::error::DeveloperError::NotFound(id))?;

        Ok(developer)
    }
}
