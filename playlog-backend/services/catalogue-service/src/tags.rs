pub mod error;
pub mod model;
pub mod repository;

use error::{Result, TagError};
pub use model::Tag;
pub use repository::{PostgresTagRepository, TagRepository};
