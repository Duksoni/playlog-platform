use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateGameRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(min = 1, max = 5000))]
    pub description: String,
    pub released: Option<NaiveDate>,
    #[validate(url)]
    pub website: Option<String>,
    #[serde(rename = "developers")]
    #[validate(length(min = 1, max = 50))]
    pub developer_ids: Vec<i32>,
    #[serde(rename = "publishers")]
    #[validate(length(min = 1, max = 50))]
    pub publisher_ids: Vec<i32>,
    #[serde(rename = "genres")]
    #[validate(length(min = 1, max = 50))]
    pub genre_ids: Vec<i32>,
    #[serde(rename = "platforms")]
    #[validate(length(min = 1, max = 50))]
    pub platform_ids: Vec<i32>,
    #[serde(rename = "tags")]
    #[validate(length(min = 1, max = 50))]
    pub tag_ids: Vec<i32>,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct UpdateGameRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(min = 1, max = 5000))]
    pub description: String,
    pub released: Option<NaiveDate>,
    #[validate(url)]
    pub website: Option<String>,
    #[validate(range(min = 0))]
    pub version: i64,
    #[serde(rename = "developers")]
    #[validate(length(max = 50))]
    pub developer_ids: Option<Vec<i32>>,
    #[serde(rename = "publishers")]
    #[validate(length(max = 50))]
    pub publisher_ids: Option<Vec<i32>>,
    #[serde(rename = "genres")]
    #[validate(length(max = 50))]
    pub genre_ids: Option<Vec<i32>>,
    #[serde(rename = "platforms")]
    #[validate(length(max = 50))]
    pub platform_ids: Option<Vec<i32>>,
    #[serde(rename = "tags")]
    #[validate(length(max = 50))]
    pub tag_ids: Option<Vec<i32>>,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct PublishGameRequest {
    #[validate(range(min = 0))]
    pub version: i64,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct DeleteGameRequest {
    #[validate(range(min = 0))]
    pub version: i64,
}

#[derive(Validate, Deserialize, IntoParams)]
pub struct PublsherGamesQuery {
    #[serde(default = "default_page")]
    #[validate(range(min = 1, max = 1000))]
    pub page: u64,
}

#[derive(Debug, Validate, Deserialize, IntoParams)]
pub struct GetGamesQuery {
    #[serde(default, rename = "gameIds")]
    #[validate(length(max = 100))]
    pub game_ids: Vec<i32>,
}

fn default_page() -> u64 {
    1
}

fn default_limit() -> u64 {
    10
}

#[derive(Debug, Validate, Deserialize, IntoParams)]
pub struct NewGameReleasesQuery {
    #[serde(default = "default_limit")]
    #[validate(range(min = 1, max = 50))]
    #[param(required = false, example = "5")]
    pub limit: u64,
}

#[derive(Validate, Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GameFilterQuery {
    #[validate(length(max = 100))]
    pub name: Option<String>,

    #[serde(default, rename = "platforms")]
    #[validate(length(max = 50))]
    #[param(rename = "platforms", required = false, style = Form, explode = true)]
    pub platform_ids: Vec<i32>,

    #[serde(default, rename = "genres")]
    #[validate(length(max = 50))]
    #[param(rename = "genres", required = false, style = Form, explode = true)]
    pub genre_ids: Vec<i32>,

    #[serde(default, rename = "tags")]
    #[validate(length(max = 50))]
    #[param(rename = "tags", required = false, style = Form, explode = true)]
    pub tag_ids: Vec<i32>,

    #[serde(default = "default_page")]
    #[validate(range(min = 1, max = 1000))]
    #[param(required = false, example = "1")]
    pub page: u64,

    #[param(
        rename = "sort",
        required = false,
        value_type = String,
        example = "name"
    )]
    pub sort: Option<GameSortField>,

    #[serde(rename = "sortDirection")]
    #[param(
        rename = "sortDirection",
        required = false,
        value_type = String,
        example = "asc"
    )]
    pub sort_direction: Option<SortDirection>,
}

#[derive(Deserialize, ToSchema, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum GameSortField {
    Name,
    Released,
}

#[derive(Deserialize, ToSchema, Clone, Copy, Debug)]
#[serde(rename_all = "snake_case")]
pub enum SortDirection {
    Asc,
    Desc,
}
