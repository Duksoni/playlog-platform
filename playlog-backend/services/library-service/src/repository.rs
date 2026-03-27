use crate::{
    error::{LibraryError, Result},
    model::{GameLibraryStatus, LibraryGame, UserGame},
};
use async_trait::async_trait;
use sqlx::{query, query_as, PgPool};
use std::collections::HashMap;
use uuid::Uuid;

#[async_trait]
pub trait LibraryRepository: Send + Sync {
    async fn get_user_library(
        &self,
        user_id: Uuid,
        status: Option<GameLibraryStatus>,
    ) -> Result<Vec<LibraryGame>>;
    async fn upsert_game(
        &self,
        user_id: Uuid,
        game_id: i32,
        status: GameLibraryStatus,
    ) -> Result<UserGame>;
    async fn get_library_stats(&self, user_id: Uuid) -> Result<HashMap<GameLibraryStatus, i64>>;
    async fn remove_game(&self, user_id: Uuid, game_id: i32) -> Result<()>;
}

pub struct PostgresLibraryRepository {
    pool: PgPool,
}

impl PostgresLibraryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl LibraryRepository for PostgresLibraryRepository {
    async fn get_user_library(
        &self,
        user_id: Uuid,
        status: Option<GameLibraryStatus>,
    ) -> Result<Vec<LibraryGame>> {
        let games = query_as!(
            LibraryGame,
            r#"
            SELECT game_id, status AS "status: GameLibraryStatus", added_at, last_updated
            FROM user_games
            WHERE user_id = $1 AND ($2::game_library_status IS NULL OR status = $2)
            ORDER BY last_updated DESC
            "#,
            user_id,
            status as _
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(games)
    }

    async fn upsert_game(
        &self,
        user_id: Uuid,
        game_id: i32,
        status: GameLibraryStatus,
    ) -> Result<UserGame> {
        let game = query_as!(
            UserGame,
            r#"
            INSERT INTO user_games (user_id, game_id, status)
            VALUES ($1, $2, $3)
            ON CONFLICT (user_id, game_id) DO UPDATE
            SET status = EXCLUDED.status, last_updated = now()
            RETURNING user_id, game_id, status AS "status: GameLibraryStatus", added_at, last_updated
            "#,
            user_id,
            game_id,
            status as _
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(game)
    }

    async fn get_library_stats(&self, user_id: Uuid) -> Result<HashMap<GameLibraryStatus, i64>> {
        let rows = query!(
            r#"
            SELECT status AS "status: GameLibraryStatus", count(*) as "count!"
            FROM user_games
            WHERE user_id = $1
            GROUP BY status
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(rows.into_iter().map(|r| (r.status, r.count)).collect())
    }

    async fn remove_game(&self, user_id: Uuid, game_id: i32) -> Result<()> {
        let result = query!(
            "DELETE FROM user_games WHERE user_id = $1 AND game_id = $2",
            user_id,
            game_id
        )
        .execute(&self.pool)
        .await?;

        if result.rows_affected() == 0 {
            return Err(LibraryError::NotFound);
        }
        Ok(())
    }
}
