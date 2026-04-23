use crate::shared::AccountStatus;
use serde::Serialize;
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "accountStatus")]
    pub account_status: AccountStatus,
    pub version: i64,
}
