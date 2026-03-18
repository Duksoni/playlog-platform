use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, FromRow, ToSchema)]
pub struct GameEntity {
    pub id: i32,
    pub name: String,
}

#[derive(Clone, Copy, Debug)]
pub enum GameEntityTable {
    Developers,
    Publishers,
    Platforms,
    Genres,
    Tags,
}

impl GameEntityTable {
    pub fn table_name(&self) -> &'static str {
        match self {
            Self::Developers => "developers",
            Self::Publishers => "publishers",
            Self::Platforms => "platforms",
            Self::Genres => "genres",
            Self::Tags => "tags",
        }
    }

    pub fn entity_name(&self) -> &'static str {
        match self {
            Self::Developers => "Developer",
            Self::Publishers => "Publisher",
            Self::Platforms => "Platform",
            Self::Genres => "Genre",
            Self::Tags => "Tag",
        }
    }
}
