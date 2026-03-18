use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateUpdateGameEntityRequest {
    #[validate(length(min = 1, max = 100))]
    pub name: String,
}

#[derive(Deserialize, IntoParams)]
pub struct SearchQuery {
    /// Partial name to search for
    pub q: String,
}

#[derive(Deserialize, IntoParams)]
pub struct PagedQuery {
    pub page: u64,
}
