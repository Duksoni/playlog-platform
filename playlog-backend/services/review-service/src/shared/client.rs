use anyhow::Result;
use reqwest::Client as HttpClient;
use service_common::http_client::expect_ok_get_response;

pub async fn ensure_game_exists(
    client: &HttpClient,
    service_url: &str,
    game_id: i32,
) -> Result<()> {
    let url = format!("{}/api/games/{}", service_url, game_id);
    expect_ok_get_response(client, &url, &format!("Game with id {} not found", game_id)).await
}
