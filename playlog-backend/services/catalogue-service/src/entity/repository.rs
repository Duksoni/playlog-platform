use super::{GameEntity, GameEntityError, GameEntityPagedResponse, GameEntitySimple, GameEntityTable, Result};
use async_trait::async_trait;
use sqlx::{query_as, query_scalar, PgPool};

#[async_trait]
pub trait GameEntityRepository: Send + Sync {
    async fn get(&self, id: i32) -> Result<Option<GameEntity>>;
    async fn get_all(&self, page: u64, limit: u64) -> Result<GameEntityPagedResponse>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<GameEntitySimple>>;
    async fn exists(&self, id: i32) -> Result<()>;
    async fn create(&self, name: &str) -> Result<GameEntity>;
    async fn update_name(&self, id: i32, name: &str, version: i64) -> Result<GameEntity>;
}

#[async_trait]
pub trait DeletableGameEntityRepository: GameEntityRepository {
    async fn delete(&self, id: i32) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct PostgresGameEntityRepository {
    table: GameEntityTable,
    pool: PgPool,
}

impl PostgresGameEntityRepository {
    pub fn new(table: GameEntityTable, pool: PgPool) -> Self {
        Self { table, pool }
    }
}

#[async_trait]
impl GameEntityRepository for PostgresGameEntityRepository {
    async fn get(&self, id: i32) -> Result<Option<GameEntity>> {
        let query = format!(
            "SELECT id, name, version FROM {} WHERE id = $1",
            self.table.table_name()
        );
        let result = query_as::<_, GameEntity>(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?;
        Ok(result)
    }

    async fn get_all(&self, page: u64, limit: u64) -> Result<GameEntityPagedResponse> {
        let limit = if limit == 0 { 10 } else { limit } as i64;
        let page = page.max(1);
        let offset = (page - 1) as i64 * limit;

        let count_query = format!("SELECT COUNT(*) FROM {}", self.table.table_name());
        let total_items: i64 = query_scalar(&count_query)
            .fetch_one(&self.pool)
            .await?;

        let total_pages = (total_items as f64 / limit as f64).ceil() as i64;

        let query = format!(
            "SELECT id, name FROM {} ORDER BY name LIMIT $1 OFFSET $2",
            self.table.table_name()
        );
        let data = query_as::<_, GameEntitySimple>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;

        Ok(GameEntityPagedResponse(
            service_common::dto::PagedResponse::new(
                data,
                total_items,
                total_pages,
                page as i64,
                limit,
            ),
        ))
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<GameEntitySimple>> {
        let query_pattern = format!("%{}%", name);
        let query = format!(
            "SELECT id, name FROM {} WHERE name ILIKE $1 ORDER BY name LIMIT 30",
            self.table.table_name()
        );
        let result = query_as::<_, GameEntitySimple>(&query)
            .bind(query_pattern)
            .fetch_all(&self.pool)
            .await?;
        Ok(result)
    }

    async fn exists(&self, id: i32) -> Result<()> {
        let query = format!(
            "SELECT EXISTS(SELECT 1 FROM {} WHERE id = $1)",
            self.table.table_name()
        );
        let exists = query_scalar::<_, bool>(&query)
            .bind(id)
            .fetch_optional(&self.pool)
            .await?
            .unwrap_or(false);
        if exists {
            Ok(())
        } else {
            Err(GameEntityError::NotFound(
                self.table.entity_name().to_string(),
                id,
            ))
        }
    }

    async fn create(&self, name: &str) -> Result<GameEntity> {
        let query = format!(
            "INSERT INTO {} (name) VALUES ($1) RETURNING id, name, version",
            self.table.table_name()
        );
        let result = query_as::<_, GameEntity>(&query)
            .bind(name)
            .fetch_one(&self.pool)
            .await?;
        Ok(result)
    }

    async fn update_name(&self, id: i32, name: &str, version: i64) -> Result<GameEntity> {
        let query = format!(
            r#"
                UPDATE {} 
                SET name = $1, version = version + 1 
                WHERE id = $2 AND version = $3 
                RETURNING id, name, version
            "#,
            self.table.table_name()
        );
        let result = query_as::<_, GameEntity>(&query)
            .bind(name)
            .bind(id)
            .bind(version)
            .fetch_optional(&self.pool)
            .await?;

        match result {
            Some(entity) => Ok(entity),
            None => {
                self.exists(id).await?;
                Err(GameEntityError::Conflict(
                    self.table.entity_name().to_string(),
                    id,
                ))
            }
        }
    }
}

#[async_trait]
impl DeletableGameEntityRepository for PostgresGameEntityRepository {
    async fn delete(&self, id: i32) -> Result<()> {
        let query = format!("DELETE FROM {} WHERE id = $1", self.table.table_name());

        let result = sqlx::query(&query).bind(id).execute(&self.pool).await?;

        if result.rows_affected() > 0 {
            Ok(())
        } else {
            self.exists(id).await?;
            Err(GameEntityError::Conflict(
                self.table.entity_name().to_string(),
                id,
            ))
        }
    }
}
