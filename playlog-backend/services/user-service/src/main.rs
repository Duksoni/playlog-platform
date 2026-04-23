mod app;
mod auth;
mod config;
mod docs;
mod shared;
mod task;
mod users;

use anyhow::Context;
use dotenvy::dotenv;
use service_common::setup::{init_sqlx_db, init_tracing, shutdown_signal};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

use app::{build_app, AppState};
use auth::{AuthService, PostgresAuthRepository};
use users::{PostgresUserRepository, UserService};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    init_tracing(env!("CARGO_CRATE_NAME"));

    let env = config::load_from_environment()?;

    let pool = init_sqlx_db(&env.database_url).await?;
    sqlx::migrate!()
        .run(&pool)
        .await
        .context("Migrations failed")?;

    let auth_repo = Box::new(PostgresAuthRepository::new(pool.clone()));
    let auth_service = AuthService::new(auth_repo.clone());
    auth_service
        .ensure_admin(env.admin_bootstrap_config)
        .await?;

    let user_repo = Box::new(PostgresUserRepository::new(pool.clone()));
    let user_service = UserService::new(user_repo);

    task::schedule_token_cleanup(auth_repo.clone());

    let state = Arc::new(AppState::new(env.app_config, auth_service, user_service));
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
