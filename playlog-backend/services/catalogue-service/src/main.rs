mod app;
mod config;
mod developers;
mod docs;
mod entity;
mod games;
mod genres;
mod platforms;
mod publishers;
mod tags;

use anyhow::Context;
use dotenvy::dotenv;
use service_common::setup::{init_sqlx_db, init_tracing, shutdown_signal};
use std::{net::SocketAddr, sync::Arc};
use tracing::info;

use crate::{
    app::{build_app, AppState},
    entity::{GameEntityTable, PostgresGameEntityRepository},
    games::{GameService, PostgresGameRepository},
};

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

    let game_repository = PostgresGameRepository::new(pool.clone());
    let dev_repository =
        PostgresGameEntityRepository::new(GameEntityTable::Developers, pool.clone());
    let genre_repository = PostgresGameEntityRepository::new(GameEntityTable::Genres, pool.clone());
    let platform_repository =
        PostgresGameEntityRepository::new(GameEntityTable::Platforms, pool.clone());
    let publisher_repository =
        PostgresGameEntityRepository::new(GameEntityTable::Publishers, pool.clone());
    let tag_repository = PostgresGameEntityRepository::new(GameEntityTable::Tags, pool.clone());

    let game_service = GameService::new(
        Box::new(game_repository),
        Box::new(dev_repository.clone()),
        Box::new(publisher_repository.clone()),
        Box::new(platform_repository.clone()),
        Box::new(genre_repository.clone()),
        Box::new(tag_repository.clone()),
    );

    let state = Arc::new(AppState::new(
        env.app_config,
        game_service,
        Arc::new(dev_repository),
        Arc::new(genre_repository),
        Arc::new(platform_repository),
        Arc::new(publisher_repository),
        Arc::new(tag_repository),
    ));

    let app = build_app(state);

    let server_address = SocketAddr::from(([0, 0, 0, 0], 3001));
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
