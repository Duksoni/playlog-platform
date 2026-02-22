use serde::Serialize;
use sqlx::FromRow;
use utoipa::ToSchema;

#[derive(Serialize, FromRow, ToSchema)]
pub struct Publisher {
    pub id: i32,
    pub name: String,
}
