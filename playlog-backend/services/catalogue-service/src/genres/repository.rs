use async_trait::async_trait;
use sqlx::{query_as, PgPool};

use super::{Genre, Result};

#[async_trait]
pub trait GenreRepository: Send + Sync {
    async fn find_all(&self) -> Result<Vec<Genre>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Genre>>;
    async fn create(&self, name: &str) -> Result<Genre>;
}

#[derive(Debug, Clone)]
pub struct PostgresGenreRepository {
    pool: PgPool,
}

impl PostgresGenreRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GenreRepository for PostgresGenreRepository {
    async fn find_all(&self) -> Result<Vec<Genre>> {
        let genres = query_as!(
            Genre,
            "SELECT id, name FROM genres ORDER BY name"
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(genres)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Genre>> {
        let genre = query_as!(
            Genre,
            "SELECT id, name FROM genres WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(genre)
    }

    async fn create(&self, name: &str) -> Result<Genre> {
        let genre = query_as!(
            Genre,
            "INSERT INTO genres (name) VALUES ($1) RETURNING id, name",
            name
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(genre)
    }
}
