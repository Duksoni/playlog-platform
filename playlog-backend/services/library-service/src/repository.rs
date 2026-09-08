use crate::{
    dto::LibraryPagedResponse,
    error::{LibraryError, Result},
    model::{GameLibraryStatus, LibraryGame, UserGame},
};
use async_trait::async_trait;
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

#[async_trait]
pub trait LibraryRepository: Send + Sync {
    async fn get_user_library(
        &self,
        user_id: Uuid,
        status: Option<GameLibraryStatus>,
        page: u64,
        limit: u64,
    ) -> Result<LibraryPagedResponse>;
    async fn upsert_game(
        &self,
        user_id: Uuid,
        game_id: i32,
        status: GameLibraryStatus,
    ) -> Result<UserGame>;
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
        page: u64,
        limit: u64,
    ) -> Result<LibraryPagedResponse> {
        let page = page.clamp(1, 1000);
        let limit = limit.clamp(1, 100);
        let offset = (page.saturating_sub(1).saturating_mul(limit)) as i64;
        let total_items: i64 = query!(
            r#"
            SELECT COUNT(*) as "count!"
            FROM user_games
            WHERE user_id = $1 AND ($2::game_library_status IS NULL OR status = $2)
            "#,
            user_id,
            status as _
        )
        .fetch_one(&self.pool)
        .await?
        .count;
        let total_pages = (total_items as f64 / limit as f64).ceil() as i64;
        let games = query_as!(
            LibraryGame,
            r#"
            SELECT game_id, status AS "status: GameLibraryStatus", added_at, last_updated
            FROM user_games
            WHERE user_id = $1 AND ($2::game_library_status IS NULL OR status = $2)
            ORDER BY last_updated DESC
            LIMIT $3 OFFSET $4
            "#,
            user_id,
            status as _,
            limit as i64,
            offset
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(LibraryPagedResponse(
            service_common::dto::PagedResponse::new(
                games,
                total_items,
                total_pages,
                page as i64,
                limit as i64,
            ),
        ))
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
