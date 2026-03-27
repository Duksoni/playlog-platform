use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use crate::model::GameLibraryStatus;

#[derive(Debug, Deserialize, ToSchema)]
pub struct AddUpdateGameRequest {
    #[serde(rename = "gameId")]
    pub game_id: i32,
    pub status: GameLibraryStatus,
}

#[derive(Debug, Deserialize, IntoParams)]
pub struct LibraryFilterQuery {
    pub status: Option<GameLibraryStatus>,
}
