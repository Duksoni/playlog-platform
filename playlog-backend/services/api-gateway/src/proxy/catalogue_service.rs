pub mod entities;
pub mod games;
pub mod health;

pub use entities::router as entity_router;
pub use games::router as games_router;
pub use health::router as catalogue_health_router;
