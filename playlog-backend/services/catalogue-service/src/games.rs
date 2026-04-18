pub mod dto;
pub mod error;
pub mod handler;
pub mod model;
pub mod repository;
pub mod service;

pub use dto::{
    CreateGameRequest, GameFilterQuery, GameSortField, GetGamesQuery, NewGameReleasesQuery,
    PublishUnpublishGameRequest, PublsherGamesQuery, SortDirection, UpdateGameRequest,
};
pub use error::{GameError, Result};
pub use model::{Game, GameDetails, GameSimple};
pub use repository::{GameRepository, PostgresGameRepository};
pub use service::GameService;

use crate::entity::GameEntitySimple;
pub type Developer = GameEntitySimple;
pub type Publisher = GameEntitySimple;
pub type Platform = GameEntitySimple;
pub type Genre = GameEntitySimple;
pub type Tag = GameEntitySimple;
