pub mod error;
pub mod model;
pub mod repository;

use error::{Result, GenreError};
pub use model::Genre;
pub use repository::{GenreRepository, PostgresGenreRepository};
