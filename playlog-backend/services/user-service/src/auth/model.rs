use serde::Serialize;
use sqlx::{FromRow, Type};
use uuid::Uuid;

#[derive(Serialize, Type)]
#[sqlx(type_name = "account_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    Active,
    Blocked,
    Deactivated,
}

#[derive(Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Role {
    User,
    Moderator,
    Admin,
}

#[derive(Serialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(rename = "accountStatus")]
    pub account_status: AccountStatus,
}
