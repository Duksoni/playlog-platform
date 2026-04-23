use super::{
    dto::UpdateProfileRequest, FindUsersResponse, Result, UpdatePasswordRequest, UserDetails, UserError,
    UserRepository,
};
use crate::users::dto::UserRoleChangeResponse;
use crate::{
    shared::password::{hash_password, verify_password},
    shared::AccountStatus::*,
};
use jwt_common::Role::{self, *};
use uuid::Uuid;

pub struct UserService {
    pub repository: Box<dyn UserRepository>,
}

impl UserService {
    pub fn new(repository: Box<dyn UserRepository>) -> Self {
        Self { repository }
    }

    pub async fn find_users(
        &self,
        requester_id: Uuid,
        username: &str,
        role: Role,
    ) -> Result<FindUsersResponse> {
        let users = self
            .repository
            .find_by_partial_username(requester_id, username, role)
            .await?;
        Ok(FindUsersResponse::new(users))
    }

    pub async fn get_user_details(&self, username: String) -> Result<UserDetails> {
        self.repository.get_user_details(username).await
    }

    pub async fn update_profile(
        &self,
        user_id: Uuid,
        request: UpdateProfileRequest,
    ) -> Result<bool> {
        if request.first_name.is_none()
            && request.last_name.is_none()
            && request.birthdate.is_none()
        {
            Err(UserError::NothingToUpdate)
        } else {
            self.repository.update_profile(user_id, &request).await
        }
    }

    pub async fn update_password(
        &self,
        user_id: Uuid,
        request: UpdatePasswordRequest,
    ) -> Result<()> {
        if request.old_password == request.new_password {
            return Err(UserError::NewPasswordSameAsOld);
        }
        let hashed_password = self.repository.get_current_password(user_id).await?;
        verify_password(&request.old_password, &hashed_password)
            .map_err(|_| UserError::WrongPassword)?;

        let hashed_password =
            hash_password(&request.new_password).map_err(|_| UserError::InternalError)?;

        let updated = self
            .repository
            .update_password(user_id, &hashed_password, request.version)
            .await?;
        if updated {
            Ok(())
        } else {
            Err(UserError::InternalError)
        }
    }

    pub async fn deactivate_account(&self, user_id: Uuid) -> Result<()> {
        self.repository.deactivate_account(user_id).await
    }

    pub async fn block_user(&self, user_id: Uuid, version: i64) -> Result<()> {
        self.require_active_account(user_id).await?;
        self.repository.block_user(user_id, version).await
    }

    pub async fn promote_user(&self, user_id: Uuid, version: i64) -> Result<UserRoleChangeResponse> {
        self.require_active_account(user_id).await?;
        let user_role = self.repository.get_user_role(user_id).await?;
        let new_role = match user_role {
            User => Moderator,
            Moderator => Admin,
            Admin => {
                return Err(UserError::CantPromote(
                    user_role.as_string(),
                    Admin.as_string(),
                ));
            }
        };
        self.repository.update_user_role(user_id, new_role, version).await?;
        Ok(UserRoleChangeResponse::new(user_role, new_role))
    }

    pub async fn demote_user(&self, user_id: Uuid, version: i64) -> Result<UserRoleChangeResponse> {
        self.require_active_account(user_id).await?;
        let user_role = self.repository.get_user_role(user_id).await?;
        let new_role = match user_role {
            User => {
                return Err(UserError::CantDemote(
                    user_role.as_string(),
                    User.as_string(),
                ));
            }
            Moderator => User,
            Admin => Moderator,
        };
        self.repository.update_user_role(user_id, new_role, version).await?;
        Ok(UserRoleChangeResponse::new(user_role, new_role))
    }

    async fn require_active_account(&self, user_id: Uuid) -> Result<()> {
        let status = self.repository.get_account_status(user_id).await?;
        match status {
            Active => Ok(()),
            Blocked => Err(UserError::UserIsBlocked),
            Deactivated => Err(UserError::UserNotFound),
        }
    }
}
