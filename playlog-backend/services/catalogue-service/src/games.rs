pub mod dto;
pub mod error;
pub mod model;
pub mod repository;

pub use dto::{CreateGameRequest, UpdateGameRequest};
use error::{GameError, Result};
pub use model::{Game, GameDetail};
pub use repository::{GameRepository, PostgresGameRepository};
