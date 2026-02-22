use async_trait::async_trait;
use sqlx::{query, query_as, PgPool};

use super::{CreateGameRequest, Game, GameDetail, GameError, Result, UpdateGameRequest};
use crate::{
    developers::Developer, genres::Genre, platforms::Platform, publishers::Publisher, tags::Tag,
};

#[async_trait]
pub trait GameRepository: Send + Sync {
    async fn find_all(&self, include_drafts: bool) -> Result<Vec<Game>>;
    async fn find_by_id(&self, id: i32) -> Result<Option<Game>>;
    async fn find_detail(&self, id: i32) -> Result<Option<GameDetail>>;
    async fn search(&self, query: &str, include_drafts: bool) -> Result<Vec<Game>>;
    async fn create(&self, data: CreateGameRequest) -> Result<Game>;
    async fn update(&self, id: i32, data: UpdateGameRequest) -> Result<Game>;
    async fn set_draft(&self, id: i32, draft: bool) -> Result<Game>;
    async fn delete(&self, id: i32) -> Result<()>;

    async fn set_developers(&self, game_id: i32, developer_ids: &[i32]) -> Result<()>;
    async fn set_publishers(&self, game_id: i32, publisher_ids: &[i32]) -> Result<()>;
    async fn set_genres(&self, game_id: i32, genre_ids: &[i32]) -> Result<()>;
    async fn set_platforms(&self, game_id: i32, platform_ids: &[i32]) -> Result<()>;
    async fn set_tags(&self, game_id: i32, tag_ids: &[i32]) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct PostgresGameRepository {
    pool: PgPool,
}

impl PostgresGameRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl GameRepository for PostgresGameRepository {
    async fn find_all(&self, include_drafts: bool) -> Result<Vec<Game>> {
        let games = query_as!(
            Game,
            r#"
                SELECT id, name, description, released, website, draft
                FROM games
                WHERE ($1 OR draft = false)
                ORDER BY name
            "#,
            include_drafts
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(games)
    }

    async fn find_by_id(&self, id: i32) -> Result<Option<Game>> {
        let game = query_as!(
            Game,
            r#"
                SELECT id, name, description, released, website, draft
                FROM games
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(game)
    }

    async fn find_detail(&self, id: i32) -> Result<Option<GameDetail>> {
        let game = self.find_by_id(id).await?;

        let Some(game) = game else {
            return Ok(None);
        };

        let developers = query_as!(
            Developer,
            r#"
                SELECT d.id, d.name
                FROM developers d
                JOIN game_developers gd ON gd.developer_id = d.id
                WHERE gd.game_id = $1
                ORDER BY d.name
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        let publishers = query_as!(
            Publisher,
            r#"
                SELECT p.id, p.name
                FROM publishers p
                JOIN game_publishers gp ON gp.publisher_id = p.id
                WHERE gp.game_id = $1
                ORDER BY p.name
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        let platforms = query_as!(
            Platform,
            r#"
                SELECT p.id, p.name
                FROM platforms p
                JOIN game_platforms gp ON gp.platform_id = p.id
                WHERE gp.game_id = $1
                ORDER BY p.name
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        let genres = query_as!(
            Genre,
            r#"
                SELECT g.id, g.name
                FROM genres g
                JOIN game_genres gg ON gg.genre_id = g.id
                WHERE gg.game_id = $1
                ORDER BY g.name
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        let tags = query_as!(
            Tag,
            r#"
                SELECT t.id, t.name
                FROM tags t
                JOIN game_tags gt ON gt.tag_id = t.id
                WHERE gt.game_id = $1
                ORDER BY t.name
            "#,
            id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(Some(GameDetail {
            game,
            developers,
            publishers,
            platforms,
            genres,
            tags,
        }))
    }

    async fn search(&self, query: &str, include_drafts: bool) -> Result<Vec<Game>> {
        let games = query_as!(
            Game,
            r#"
                SELECT id, name, description, released, website, draft
                FROM games
                WHERE name ILIKE $1
                AND ($2 OR draft = false)
                ORDER BY name
                LIMIT 50
            "#,
            format!("%{}%", query),
            include_drafts
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(games)
    }

    async fn create(&self, data: CreateGameRequest) -> Result<Game> {
        let game = query_as!(
            Game,
            r#"
                INSERT INTO games (name, description, released, website)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, description, released, website, draft
            "#,
            data.name,
            data.description,
            data.released,
            data.website
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(game)
    }

    async fn update(&self, id: i32, data: UpdateGameRequest) -> Result<Game> {
        let game = query_as!(
            Game,
            r#"
                UPDATE games
                SET name = $1, description = $2, released = $3, website = $4
                WHERE id = $5
                RETURNING id, name, description, released, website, draft
             "#,
            data.name,
            data.description,
            data.released,
            data.website,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(GameError::NotFound(id))?;

        Ok(game)
    }

    async fn set_draft(&self, id: i32, draft: bool) -> Result<Game> {
        let game = query_as!(
            Game,
            r#"
                UPDATE games 
                SET draft = $1 
                WHERE id = $2
                RETURNING id, name, description, released, website, draft
             "#,
            draft,
            id
        )
        .fetch_optional(&self.pool)
        .await?
        .ok_or(GameError::NotFound(id))?;

        Ok(game)
    }

    async fn delete(&self, id: i32) -> Result<()> {
        let result = query!("DELETE FROM games WHERE id = $1", id)
            .execute(&self.pool)
            .await?;

        if result.rows_affected() == 0 {
            return Err(GameError::NotFound(id));
        }

        Ok(())
    }

    async fn set_developers(&self, game_id: i32, developer_ids: &[i32]) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        query!("DELETE FROM game_developers WHERE game_id = $1", game_id)
            .execute(&mut *transaction)
            .await?;

        for developer_id in developer_ids {
            query!(
                "INSERT INTO game_developers (game_id, developer_id) VALUES ($1, $2)",
                game_id,
                developer_id
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    async fn set_publishers(&self, game_id: i32, publisher_ids: &[i32]) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        query!("DELETE FROM game_publishers WHERE game_id = $1", game_id)
            .execute(&mut *transaction)
            .await?;

        for publisher_id in publisher_ids {
            query!(
                "INSERT INTO game_publishers (game_id, publisher_id) VALUES ($1, $2)",
                game_id,
                publisher_id
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    async fn set_genres(&self, game_id: i32, genre_ids: &[i32]) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        query!("DELETE FROM game_genres WHERE game_id = $1", game_id)
            .execute(&mut *transaction)
            .await?;

        for genre_id in genre_ids {
            query!(
                "INSERT INTO game_genres (game_id, genre_id) VALUES ($1, $2)",
                game_id,
                genre_id
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    async fn set_platforms(&self, game_id: i32, platform_ids: &[i32]) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        query!("DELETE FROM game_platforms WHERE game_id = $1", game_id)
            .execute(&mut *transaction)
            .await?;

        for platform_id in platform_ids {
            query!(
                "INSERT INTO game_platforms (game_id, platform_id) VALUES ($1, $2)",
                game_id,
                platform_id
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }

    async fn set_tags(&self, game_id: i32, tag_ids: &[i32]) -> Result<()> {
        let mut transaction = self.pool.begin().await?;

        query!("DELETE FROM game_tags WHERE game_id = $1", game_id)
            .execute(&mut *transaction)
            .await?;

        for tag_id in tag_ids {
            query!(
                "INSERT INTO game_tags (game_id, tag_id) VALUES ($1, $2)",
                game_id,
                tag_id
            )
            .execute(&mut *transaction)
            .await?;
        }

        transaction.commit().await?;
        Ok(())
    }
}
