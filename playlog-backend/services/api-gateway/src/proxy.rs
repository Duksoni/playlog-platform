pub mod catalogue_service;
pub mod client;
pub mod handler;
pub mod library_service;
pub mod multimedia_service;
pub mod review_service;
pub mod state;
pub mod user_service;

pub use client::ProxyClient;
pub use handler::proxy_handler;
pub use state::ServiceAppState;
