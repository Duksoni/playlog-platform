use super::{GameEntity, GameEntityError, GameEntitySimple, GameEntityTable, Result};
use async_trait::async_trait;
use sqlx::{query_as, query_scalar, PgPool};

#[async_trait]
pub trait GameEntityRepository: Send + Sync {
    async fn get(&self, id: i32) -> Result<Option<GameEntity>>;
    async fn get_all_paged(&self, page: u64) -> Result<Vec<GameEntitySimple>>;
    async fn find_by_name(&self, name: &str) -> Result<Vec<GameEntitySimple>>;
    async fn exists(&self, id: i32) -> Result<()>;
    async fn create(&self, name: &str) -> Result<GameEntity>;
    async fn update_name(&self, id: i32, name: &str, version: i64) -> Result<GameEntity>;
}

#[async_trait]
pub trait SmallGameEntityRepository: GameEntityRepository {
    async fn get_all(&self) -> Result<Vec<GameEntitySimple>>;
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

    async fn get_all_paged(&self, page: u64) -> Result<Vec<GameEntitySimple>> {
        let limit = 10i64;
        let offset = ((page.max(1) - 1) * 10) as i64;

        let query = format!(
            "SELECT id, name FROM {} ORDER BY name LIMIT $1 OFFSET $2",
            self.table.table_name()
        );
        let result = query_as::<_, GameEntitySimple>(&query)
            .bind(limit)
            .bind(offset)
            .fetch_all(&self.pool)
            .await?;
        Ok(result)
    }

    async fn find_by_name(&self, name: &str) -> Result<Vec<GameEntitySimple>> {
        let query_pattern = format!("%{}%", name);
        let query = format!(
            "SELECT id, name FROM {} WHERE name ILIKE $1 ORDER BY name LIMIT 20",
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
impl SmallGameEntityRepository for PostgresGameEntityRepository {
    async fn get_all(&self) -> Result<Vec<GameEntitySimple>> {
        match self.table {
            GameEntityTable::Platforms | GameEntityTable::Genres => {
                let query = format!(
                    "SELECT id, name FROM {} ORDER BY name",
                    self.table.table_name()
                );
                let result = query_as::<_, GameEntitySimple>(&query)
                    .fetch_all(&self.pool)
                    .await?;
                Ok(result)
            }
            _ => Err(GameEntityError::UnsupportedOperation(
                self.table.entity_name().to_string(),
                "get_all".to_string(),
            )),
        }
    }
}
