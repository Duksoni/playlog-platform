use crate::entity::GameEntitySimple;
use serde::{Deserialize, Serialize};
use service_common::dto::PagedResponse;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateGameEntityRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct UpdateGameEntityRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
    #[validate(range(min = 0))]
    pub version: i64,
}

#[derive(Serialize, ToSchema)]
#[serde(transparent)]
pub struct GameEntityPagedResponse(pub PagedResponse<GameEntitySimple>);

fn default_page() -> u64 {
    1
}

fn default_limit() -> u64 {
    10
}

#[derive(Validate, Deserialize, IntoParams)]
pub struct SearchQuery {
    #[validate(length(min = 1, max = 100))]
    pub q: String,
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 50))]
    #[param(required = false, example = "10")]
    pub limit: u64,
}

#[derive(Validate, Deserialize, IntoParams)]
pub struct PagedQuery {
    #[serde(default = "default_page")]
    #[validate(range(min = 1, max = 1000))]
    #[param(required = false, example = "1")]
    pub page: u64,
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 50))]
    #[param(required = false, example = "10")]
    pub limit: u64,
}
