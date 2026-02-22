use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateGameRequest {
    pub name: String,
    pub description: String,
    pub released: Option<NaiveDate>,
    pub website: Option<String>,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct UpdateGameRequest {
    pub name: String,
    pub description: String,
    pub released: Option<NaiveDate>,
    pub website: Option<String>,
}
