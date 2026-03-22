use crate::{
    developers::Developer, genres::Genre, platforms::Platform, publishers::Publisher, tags::Tag,
};
use chrono::NaiveDate;
use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, FromRow, ToSchema)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub released: Option<NaiveDate>,
    pub website: Option<String>,
    pub draft: bool,
}

#[derive(Serialize, FromRow, ToSchema, Default)]
pub struct GameSimple {
    pub id: i32,
    pub name: String,
    pub released: Option<NaiveDate>,
    pub draft: bool,
}

#[derive(Serialize, ToSchema)]
pub struct GameDetails {
    #[serde(flatten)]
    pub game: Game,
    pub developers: Vec<Developer>,
    pub publishers: Vec<Publisher>,
    pub platforms: Vec<Platform>,
    pub genres: Vec<Genre>,
    pub tags: Vec<Tag>,
}

impl GameDetails {
    pub fn new(
        game: Game,
        developers: Vec<Developer>,
        publishers: Vec<Publisher>,
        platforms: Vec<Platform>,
        genres: Vec<Genre>,
        tags: Vec<Tag>,
    ) -> Self {
        Self {
            game,
            developers,
            publishers,
            platforms,
            genres,
            tags,
        }
    }
}
