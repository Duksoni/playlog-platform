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

#[derive(Deserialize, IntoParams)]
pub struct SearchQuery {
    /// Partial name to search for
    pub q: String,
    #[param(required = false, example = "10")]
    pub limit: u64
}

#[derive(Deserialize, IntoParams)]
pub struct PagedQuery {
    #[serde(default)]
    #[param(required = false, example = "1")]
    pub page: u64,
    #[serde(default)]
    #[param(required = false, example = "10")]
    pub limit: u64,
}
