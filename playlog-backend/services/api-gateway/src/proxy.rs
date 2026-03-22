pub mod catalogue_service;
pub mod state;
pub mod multimedia_service;
pub mod client;
pub mod handler;
pub mod user_service;

pub use state::ServiceAppState;
pub use client::ProxyClient;
pub use handler::proxy_handler;
