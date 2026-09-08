use crate::model::{GameLibraryStatus, LibraryGame};
use serde::{Deserialize, Serialize};
use service_common::dto::PagedResponse;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Debug, Validate, Deserialize, ToSchema)]
pub struct AddUpdateGameRequest {
    #[serde(rename = "gameId")]
    #[validate(range(min = 1))]
    pub game_id: i32,
    pub status: GameLibraryStatus,
}

fn default_page() -> u64 {
    1
}

fn default_limit() -> u64 {
    10
}

#[derive(Serialize, ToSchema)]
#[serde(transparent)]
pub struct LibraryPagedResponse(pub PagedResponse<LibraryGame>);

#[derive(Debug, Validate, Deserialize, IntoParams)]
pub struct LibraryFilterQuery {
    pub status: Option<GameLibraryStatus>,
    #[serde(default = "default_page")]
    #[validate(range(min = 1, max = 1000))]
    #[param(required = false, example = "1")]
    pub page: u64,
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 100))]
    #[param(required = false, example = "20")]
    pub limit: u64,
}
