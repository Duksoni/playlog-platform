use super::SimpleUser;
use crate::shared::{validate_birthdate_range, validate_first_name, validate_password};
use chrono::NaiveDate;
use jwt_common::Role;
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use validator::Validate;

#[derive(Validate, Deserialize, ToSchema)]
pub struct UpdatePasswordRequest {
    #[validate(length(min = 1, max = 64))]
    #[serde(rename = "oldPassword")]
    pub old_password: String,
    #[validate(custom(function = "validate_password"))]
    #[serde(rename = "newPassword")]
    pub new_password: String,
}

#[derive(Validate, Deserialize, ToSchema)]
pub struct UpdateProfileRequest {
    #[validate(custom(function = "validate_first_name"))]
    #[serde(rename = "firstName")]
    pub first_name: Option<String>,
    #[validate(custom(function = "validate_first_name"))]
    #[serde(rename = "lastName")]
    pub last_name: Option<String>,
    #[validate(custom(function = "validate_birthdate_range"))]
    pub birthdate: Option<NaiveDate>,
}

#[derive(Validate, Deserialize, IntoParams)]
pub struct FindUsersQuery {
    #[validate(length(min = 2, max = 16))]
    pub partial_username: String,
    pub role: Role,
}

#[derive(Serialize, ToSchema)]
pub struct FindUsersResponse {
    pub users: Vec<SimpleUser>,
}

impl FindUsersResponse {
    pub fn new(users: Vec<SimpleUser>) -> Self {
        Self { users }
    }
}

#[derive(Serialize, ToSchema)]
pub struct UserRoleChangeResponse {
    #[serde(rename = "oldRole")]
    pub old_role: Role,
    #[serde(rename = "newRole")]
    pub new_role: Role,
}

impl UserRoleChangeResponse {
    pub fn new(old_role: Role, new_role: Role) -> Self {
        Self { old_role, new_role }
    }
}
