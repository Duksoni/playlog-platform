mod app;
mod auth;
mod config;
mod setup;
mod shared;
mod task;
mod users;

use dotenvy::dotenv;
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

use app::{build_app, AppState};
use auth::{AuthService, PostgresAuthRepository};
use setup::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    init_tracing();

    dotenv().ok();
    let environment = config::load_from_environment()?;

    let pool = init_db(&environment.0).await?;

    let auth_repo = Box::new(PostgresAuthRepository::new(pool.clone()));
    let auth_service = AuthService::new(auth_repo.clone());
    auth_service.ensure_admin(environment.2).await?;

    task::schedule_token_cleanup(auth_repo.clone());

    let state = Arc::new(AppState::new(environment.1, auth_service));
    let app = build_app(state);

    let server_address = SocketAddr::from(([0, 0, 0, 0], 3005));
    let listener = tokio::net::TcpListener::bind(&server_address).await?;

    info!(
        "Server started. View docs at http://{}/docs",
        server_address
    );
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await?;
    Ok(())
}
