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

#[derive(Serialize, ToSchema)]
pub struct GameDetail {
    #[serde(flatten)]
    pub game: Game,
    pub developers: Vec<Developer>,
    pub publishers: Vec<Publisher>,
    pub platforms: Vec<Platform>,
    pub genres: Vec<Genre>,
    pub tags: Vec<Tag>,
}
