pub mod error;
pub mod model;
pub mod repository;

use error::{Result, PlatformError};
pub use model::Platform;
pub use repository::{PlatformRepository, PostgresPlatformRepository};
