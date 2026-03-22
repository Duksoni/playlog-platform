use chrono::NaiveDate;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
pub struct CreateUpdateGameRequest {
    #[validate(length(min = 1, max = 200))]
    pub name: String,
    #[validate(length(min = 1))]
    pub description: String,
    pub released: Option<NaiveDate>,
    #[validate(url)]
    pub website: Option<String>,

    // Associated entities - may be empty but must be valid ids
    #[serde(rename = "developers")]
    pub developer_ids: Vec<i32>,
    #[serde(rename = "publishers")]
    pub publisher_ids: Vec<i32>,
    #[serde(rename = "genres")]
    pub genre_ids: Vec<i32>,
    #[serde(rename = "platforms")]
    pub platform_ids: Vec<i32>,
    #[serde(rename = "tags")]
    pub tag_ids: Vec<i32>,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct GameFilterQuery {
    pub name: Option<String>,

    #[serde(default)]
    #[param(rename = "platforms", required = false, style = Form, explode = true)]
    pub platform_ids: Vec<i32>,

    #[serde(default)]
    #[param(rename = "genres", required = false, style = Form, explode = true)]
    pub genre_ids: Vec<i32>,

    #[serde(default)]
    #[param(rename = "tags", required = false, style = Form, explode = true)]
    pub tag_ids: Vec<i32>,

    #[serde(default)]
    #[param(required = false, example = "0")]
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
