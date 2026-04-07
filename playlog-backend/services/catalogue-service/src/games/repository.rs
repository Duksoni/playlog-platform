use super::{
    CreateGameRequest, Developer, Game, GameDetails, GameError, GameFilterQuery, GameSimple,
    GameSortField, Genre, Platform, Publisher, Result, SortDirection, Tag, UpdateGameRequest,
};
use async_trait::async_trait;
use sqlx::{query, query_as, query_scalar, PgPool, Postgres, Transaction};

#[async_trait]
pub trait GameRepository: Send + Sync {
    async fn filter(
        &self,
        include_drafts: bool,
        params: GameFilterQuery,
    ) -> Result<Vec<GameSimple>>;
    async fn find_by_developer(&self, developer_id: i32) -> Result<Vec<GameSimple>>;
    async fn find_by_publisher(&self, publisher_id: i32, page: u64) -> Result<Vec<GameSimple>>;
    async fn get_all_unpublished(&self) -> Result<Vec<GameSimple>>;
    async fn get(&self, id: i32, include_draft: bool) -> Result<Option<Game>>;
    async fn get_details(&self, id: i32, include_draft: bool) -> Result<Option<GameDetails>>;
    async fn create(&self, data: CreateGameRequest) -> Result<Game>;
    async fn update(&self, id: i32, data: UpdateGameRequest) -> Result<GameDetails>;
    async fn set_draft(&self, id: i32, draft: bool, version: i64) -> Result<Game>;
    async fn delete(&self, id: i32) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct PostgresGameRepository {
    pool: PgPool,
}

#[async_trait]
impl GameRepository for PostgresGameRepository {
    async fn filter(
        &self,
        include_drafts: bool,
        params: GameFilterQuery,
    ) -> Result<Vec<GameSimple>> {
        let query = self.build_filter_query(include_drafts, &params);
        let games: Vec<GameSimple> = query_as(&query).fetch_all(&self.pool).await?;
        Ok(games)
    }

    async fn find_by_developer(&self, developer_id: i32) -> Result<Vec<GameSimple>> {
        let games = query_as!(
            GameSimple,
            r#"
                SELECT id, name, released, draft
                FROM games
                JOIN game_developers gd ON gd.game_id = games.id
                WHERE gd.developer_id = $1
                ORDER BY name
            "#,
            developer_id
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(games)
    }

    async fn find_by_publisher(&self, publisher_id: i32, page: u64) -> Result<Vec<GameSimple>> {
        let offset = (page.max(1) - 1) * 10;
        let games = query_as!(
            GameSimple,
            r#"
                SELECT id, name, released, draft
                FROM games JOIN game_publishers gp ON gp.game_id = games.id
                WHERE gp.publisher_id = $1
                ORDER BY name
                LIMIT 10
                OFFSET $2
            "#,
            publisher_id,
            offset as i64
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(games)
    }

    async fn get_all_unpublished(&self) -> Result<Vec<GameSimple>> {
        let games = query_as!(
            GameSimple,
            r#"
                SELECT id, name, released, draft
                FROM games
                WHERE draft = true
                ORDER BY name
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(games)
    }

    async fn get(&self, id: i32, include_draft: bool) -> Result<Option<Game>> {
        let game = query_as!(
            Game,
            r#"
                SELECT id, name, description, released, website, draft, version
                FROM games
                WHERE id = $1 AND ($2 OR draft = false)
            "#,
            id,
            include_draft
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(game)
    }

    async fn get_details(&self, id: i32, include_draft: bool) -> Result<Option<GameDetails>> {
        let game = self.get(id, include_draft).await?;

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

        Ok(Some(GameDetails::new(
            game, developers, publishers, platforms, genres, tags,
        )))
    }

    async fn create(&self, data: CreateGameRequest) -> Result<Game> {
        let mut transaction = self.pool.begin().await?;

        let game = query_as!(
            Game,
            r#"
                INSERT INTO games (name, description, released, website)
                VALUES ($1, $2, $3, $4)
                RETURNING id, name, description, released, website, draft, version
            "#,
            data.name,
            data.description,
            data.released,
            data.website
        )
        .fetch_one(&mut *transaction)
        .await?;
        self.set_developers(&mut transaction, game.id, &data.developer_ids)
            .await?;
        self.set_publishers(&mut transaction, game.id, &data.publisher_ids)
            .await?;
        self.set_genres(&mut transaction, game.id, &data.genre_ids)
            .await?;
        self.set_platforms(&mut transaction, game.id, &data.platform_ids)
            .await?;
        self.set_tags(&mut transaction, game.id, &data.tag_ids)
            .await?;

        transaction.commit().await?;

        Ok(game)
    }

    async fn update(&self, id: i32, data: UpdateGameRequest) -> Result<GameDetails> {
        let mut transaction = self.pool.begin().await?;

        let game = query_as!(
            Game,
            r#"
                UPDATE games
                SET name = COALESCE($1, name), 
                    description = COALESCE($2, description), 
                    released = COALESCE($3, released), 
                    website = COALESCE($4, website),
                    version = version + 1
                WHERE id = $5 AND version = $6
                RETURNING id, name, description, released, website, draft, version
             "#,
            data.name,
            data.description,
            data.released,
            data.website,
            id,
            data.version
        )
        .fetch_optional(&mut *transaction)
        .await?;

        if game.is_none() {
            let exists = query_scalar!("SELECT EXISTS(SELECT 1 FROM games WHERE id = $1)", id)
                .fetch_one(&mut *transaction)
                .await?
                .unwrap_or(false);

            return if exists {
                Err(GameError::Conflict(id))
            } else {
                Err(GameError::NotFound(id))
            }
        }

        if let Some(developer_ids) = data.developer_ids {
            self.set_developers(&mut transaction, id, &developer_ids)
                .await?;
        }
        if let Some(publisher_ids) = data.publisher_ids {
            self.set_publishers(&mut transaction, id, &publisher_ids)
                .await?;
        }
        if let Some(genre_ids) = data.genre_ids {
            self.set_genres(&mut transaction, id, &genre_ids).await?;
        }
        if let Some(platform_ids) = data.platform_ids {
            self.set_platforms(&mut transaction, id, &platform_ids)
                .await?;
        }
        if let Some(tag_ids) = data.tag_ids {
            self.set_tags(&mut transaction, id, &tag_ids).await?;
        }

        transaction.commit().await?;

        self.get_details(id, true)
            .await?
            .ok_or(GameError::NotFound(id))
    }

    async fn set_draft(&self, id: i32, draft: bool, version: i64) -> Result<Game> {
        let game = query_as!(
            Game,
            r#"
                UPDATE games 
                SET draft = $1, version = version + 1
                WHERE id = $2 AND version = $3
                RETURNING id, name, description, released, website, draft, version
             "#,
            draft,
            id,
            version
        )
        .fetch_optional(&self.pool)
        .await?;

        match game {
            Some(g) => Ok(g),
            None => {
                let exists = query_scalar!("SELECT EXISTS(SELECT 1 FROM games WHERE id = $1)", id)
                    .fetch_one(&self.pool)
                    .await?
                    .unwrap_or(false);
                if exists {
                    Err(GameError::Conflict(id))
                } else {
                    Err(GameError::NotFound(id))
                }
            }
        }
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
}

impl PostgresGameRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    fn build_filter_query(&self, include_drafts: bool, params: &GameFilterQuery) -> String {
        let mut query = String::from(
            r#"
            SELECT DISTINCT g.id, g.name, g.released, g.draft
            FROM games g
                LEFT JOIN game_tags gt ON gt.game_id = g.id
                LEFT JOIN game_genres gg ON gg.game_id = g.id
                LEFT JOIN game_platforms gp ON gp.game_id = g.id
        "#,
        );
        if !include_drafts {
            query.push_str(" WHERE g.draft = false ");
        }

        if !params.platform_ids.is_empty() {
            let id_str = params
                .platform_ids
                .iter()
                .map(|id| format!("{}", id))
                .collect::<Vec<String>>()
                .join(", ");
            query.push_str(&format!(" AND gp.platform_id IN ({})", id_str));
        }
        if !params.genre_ids.is_empty() {
            let id_str = params
                .genre_ids
                .iter()
                .map(|id| format!("{}", id))
                .collect::<Vec<String>>()
                .join(", ");
            query.push_str(&format!(" AND gg.genre_id IN ({})", id_str));
        }
        if !params.tag_ids.is_empty() {
            let id_str = params
                .tag_ids
                .iter()
                .map(|id| format!("{}", id))
                .collect::<Vec<String>>()
                .join(", ");
            query.push_str(&format!(" AND gt.tag_id IN ({})", id_str));
        }
        if let Some(name) = &params.name {
            query.push_str(" AND g.name ILIKE ");
            query.push_str(&format!("'%{}%'", name));
        }

        let sort_field = match params.sort.unwrap_or(GameSortField::Name) {
            GameSortField::Name => "g.name",
            GameSortField::Released => "g.released",
        };

        let direction = match params.sort_direction.unwrap_or(SortDirection::Asc) {
            SortDirection::Asc => "ASC",
            SortDirection::Desc => "DESC",
        };

        query.push_str(&format!(" ORDER BY {} {}", sort_field, direction));
        query.push_str(" LIMIT 10");
        query.push_str(" OFFSET ");
        let offset = (params.page.max(1) - 1) * 10;
        query.push_str(&format!("{}", offset));
        query
    }

    async fn set_developers(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        game_id: i32,
        developer_ids: &[i32],
    ) -> Result<()> {
        query!("DELETE FROM game_developers WHERE game_id = $1", game_id)
            .execute(&mut **transaction)
            .await?;

        for developer_id in developer_ids {
            query!(
                "INSERT INTO game_developers (game_id, developer_id) VALUES ($1, $2)",
                game_id,
                developer_id
            )
            .execute(&mut **transaction)
            .await?;
        }
        Ok(())
    }

    async fn set_publishers(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        game_id: i32,
        publisher_ids: &[i32],
    ) -> Result<()> {
        query!("DELETE FROM game_publishers WHERE game_id = $1", game_id)
            .execute(&mut **transaction)
            .await?;

        for publisher_id in publisher_ids {
            query!(
                "INSERT INTO game_publishers (game_id, publisher_id) VALUES ($1, $2)",
                game_id,
                publisher_id
            )
            .execute(&mut **transaction)
            .await?;
        }
        Ok(())
    }

    async fn set_genres(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        game_id: i32,
        genre_ids: &[i32],
    ) -> Result<()> {
        query!("DELETE FROM game_genres WHERE game_id = $1", game_id)
            .execute(&mut **transaction)
            .await?;

        for genre_id in genre_ids {
            query!(
                "INSERT INTO game_genres (game_id, genre_id) VALUES ($1, $2)",
                game_id,
                genre_id
            )
            .execute(&mut **transaction)
            .await?;
        }
        Ok(())
    }

    async fn set_platforms(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        game_id: i32,
        platform_ids: &[i32],
    ) -> Result<()> {
        query!("DELETE FROM game_platforms WHERE game_id = $1", game_id)
            .execute(&mut **transaction)
            .await?;

        for platform_id in platform_ids {
            query!(
                "INSERT INTO game_platforms (game_id, platform_id) VALUES ($1, $2)",
                game_id,
                platform_id
            )
            .execute(&mut **transaction)
            .await?;
        }
        Ok(())
    }

    async fn set_tags(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        game_id: i32,
        tag_ids: &[i32],
    ) -> Result<()> {
        query!("DELETE FROM game_tags WHERE game_id = $1", game_id)
            .execute(&mut **transaction)
            .await?;

        for tag_id in tag_ids {
            query!(
                "INSERT INTO game_tags (game_id, tag_id) VALUES ($1, $2)",
                game_id,
                tag_id
            )
            .execute(&mut **transaction)
            .await?;
        }
        Ok(())
    }
}
