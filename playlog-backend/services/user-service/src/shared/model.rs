use serde::Serialize;
use sqlx::Type;

#[derive(Debug, Copy, Clone, Type, Serialize)]
#[sqlx(type_name = "account_status", rename_all = "SCREAMING_SNAKE_CASE")]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum AccountStatus {
    Active,
    Blocked,
    Deactivated,
}
