pub mod error;
pub mod model;
pub mod repository;

use error::{Result, PublisherError};
pub use model::Publisher;
pub use repository::{PostgresPublisherRepository, PublisherRepository};
