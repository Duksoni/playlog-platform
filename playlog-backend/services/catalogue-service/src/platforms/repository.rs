use async_trait::async_trait;
use sqlx::{query_as, PgPool};

use super::{Platform, Result};

#[async_trait]
pub trait PlatformRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Platform>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Platform>>;
    async fn create(&self, name: &str) -> Result<Platform>;
}

#[derive(Debug, Clone)]
pub struct PostgresPlatformRepository {
    pool: PgPool,
}

impl PostgresPlatformRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl PlatformRepository for PostgresPlatformRepository {
    async fn find_all(&self) -> Result<Vec<Platform>> {
        let platforms = query_as!(
            Platform,
            "SELECT id, name FROM platforms ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(platforms)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Platform>> {
        let platform = query_as!(
            Platform,
            "SELECT id, name FROM platforms WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(platform)
    }

    async fn create(&self, name: &str) -> Result<Platform> {
        let platform = query_as!(
            Platform,
            "INSERT INTO platforms (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(platform)
    }
}
