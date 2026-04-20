use super::{Result, SimpleUser, UpdateProfileRequest, UserDetails, UserError};
use crate::shared::AccountStatus;
use async_trait::async_trait;
use jwt_common::Role;
use sqlx::{query, query_as, query_scalar, PgPool};
use std::str::FromStr;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_partial_username(
        &self,
        requester_id: Uuid,
        username: &str,
        role: Role,
    ) -> Result<Vec<SimpleUser>>;
    async fn get_user_details(&self, username: String) -> Result<UserDetails>;
    async fn get_current_password(&self, user_id: Uuid) -> Result<String>;
    async fn get_user_role(&self, user_id: Uuid) -> Result<Role>;
    async fn get_account_status(&self, user_id: Uuid) -> Result<AccountStatus>;
    async fn update_profile(&self, user_id: Uuid, request: &UpdateProfileRequest) -> Result<bool>;
    async fn update_password(&self, user_id: Uuid, new_password: &str, version: i64) -> Result<bool>;
    async fn update_user_role(&self, user_id: Uuid, new_role: Role, version: i64) -> Result<()>;
    async fn deactivate_account(&self, user_id: Uuid) -> Result<()>;
    async fn block_user(&self, user_id: Uuid, version: i64) -> Result<()>;
}

#[derive(Debug, Clone)]
pub struct PostgresUserRepository {
    pool: PgPool,
}

#[async_trait]
impl UserRepository for PostgresUserRepository {
    async fn find_by_partial_username(
        &self,
        requester_id: Uuid,
        username: &str,
        role: Role,
    ) -> Result<Vec<SimpleUser>> {
        let users = query_as!(
            SimpleUser,
            r#"
                SELECT u.id, username, version
                FROM users u
                         JOIN user_roles ur ON ur.user_id = u.id
                         JOIN roles r ON r.id = ur.role_id
                WHERE u.id != $1 AND username LIKE $2
                  AND account_status = 'ACTIVE'
                  AND r.name = $3
                ORDER BY username
            "#,
            requester_id,
            format!("%{}%", username),
            role.as_db_value()
        )
        .fetch_all(&self.pool)
        .await?;
        Ok(users)
    }

    async fn get_user_details(&self, username: String) -> Result<UserDetails> {
        let user = query_as!(
            UserDetails,
            r#"
                SELECT u.id, username, r.name as role, first_name, last_name, birthdate, created_at, u.version
                FROM users u
                    INNER JOIN user_profiles p ON u.id = p.user_id
                    INNER JOIN user_roles ur ON ur.user_id = u.id
                    INNER JOIN roles r ON r.id = ur.role_id
                WHERE u.username = $1 AND account_status = 'ACTIVE'
            "#,
            username
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(user)
    }

    async fn get_current_password(&self, user_id: Uuid) -> Result<String> {
        let password = query_scalar!(
            r#"
                SELECT password 
                FROM users 
                WHERE id = $1 AND account_status = 'ACTIVE'
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(password)
    }

    async fn get_user_role(&self, user_id: Uuid) -> Result<Role> {
        let role = query_scalar!(
            r#"
                SELECT r.name 
                FROM roles r INNER JOIN user_roles ur ON ur.role_id = r.id 
                WHERE ur.user_id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(Role::from_str(&role).map_err(|_| UserError::InternalError)?)
    }

    async fn get_account_status(&self, user_id: Uuid) -> Result<AccountStatus> {
        let status = query_scalar!(
            r#"
                SELECT account_status AS "account_status!: AccountStatus" 
                FROM users
                WHERE id = $1
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(status)
    }

    async fn update_profile(&self, user_id: Uuid, request: &UpdateProfileRequest) -> Result<bool> {
        let mut tx = self.pool.begin().await?;

        let rows_changed = query!(
            r#"
                UPDATE users
                SET version = version + 1
                WHERE id = $1 AND version = $2
            "#,
            user_id,
            request.version
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        if rows_changed == 0 {
            return self.handle_update_failure::<bool>(user_id, &mut tx).await;
        }

        query!(
            r#"
                UPDATE user_profiles
                SET
                   first_name = COALESCE($2, first_name),
                   last_name  = COALESCE($3, last_name),
                   birthdate  = COALESCE($4, birthdate)
                WHERE user_id = $1
            "#,
            user_id,
            request.first_name,
            request.last_name,
            request.birthdate
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        Ok(true)
    }

    async fn update_password(&self, user_id: Uuid, new_password: &str, version: i64) -> Result<bool> {
        let rows_changed = query!(
            r#"
                UPDATE users
                SET password = $2, version = version + 1
                WHERE id = $1 AND version = $3
            "#,
            user_id,
            new_password,
            version
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_changed == 0 {
            let mut tx = self.pool.begin().await?;
            return self.handle_update_failure::<bool>(user_id, &mut tx).await;
        }

        Ok(true)
    }

    async fn update_user_role(&self, user_id: Uuid, new_role: Role, version: i64) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let rows_changed = query!(
            r#"
                UPDATE users
                SET version = version + 1
                WHERE id = $1 AND version = $2
            "#,
            user_id,
            version
        )
        .execute(&mut *tx)
        .await?
        .rows_affected();

        if rows_changed == 0 {
            return self.handle_update_failure::<()>(user_id, &mut tx).await;
        }

        query!(
            r#"
                UPDATE user_roles
                SET role_id = (SELECT id FROM roles WHERE name = $2)
                WHERE user_id = $1
            "#,
            user_id,
            new_role.as_db_value()
        )
        .execute(&mut *tx)
        .await?;

        tx.commit().await?;
        self.clear_refresh_tokens(user_id).await?;
        Ok(())
    }

    async fn deactivate_account(&self, user_id: Uuid) -> Result<()> {
        let mut tx = self.pool.begin().await?;

        let user_info = query!(
            r#"
                SELECT account_status AS "account_status: AccountStatus", r.name as role
                FROM users u
                    INNER JOIN user_roles ur ON ur.user_id = u.id
                    INNER JOIN roles r ON r.id = ur.role_id
                WHERE u.id = $1
                FOR UPDATE
            "#,
            user_id
        )
        .fetch_optional(&mut *tx)
        .await?;

        let user_info = match user_info {
            Some(info) => info,
            None => return Err(UserError::UserNotFound),
        };

        match user_info.account_status {
            AccountStatus::Active => (),
            AccountStatus::Blocked => return Err(UserError::UserIsBlocked),
            AccountStatus::Deactivated => return Err(UserError::UserNotFound),
        }

        if user_info.role == Role::Admin.as_db_value() {
            return Err(UserError::AdminCantDeactivateAccount);
        }

        query!(
            "UPDATE users SET account_status = 'DEACTIVATED', version = version + 1 WHERE id = $1",
            user_id
        )
        .execute(&mut *tx)
        .await?;

        query!("DELETE FROM user_tokens WHERE user_id = $1", user_id)
            .execute(&mut *tx)
            .await?;

        tx.commit().await?;
        Ok(())
    }

    async fn block_user(&self, user_id: Uuid, version: i64) -> Result<()> {
        let rows_changed = query!(
            "UPDATE users SET account_status = 'BLOCKED', version = version + 1 WHERE id = $1 AND version = $2",
            user_id,
            version
        )
        .execute(&self.pool)
        .await?
        .rows_affected();

        if rows_changed == 0 {
            let mut tx = self.pool.begin().await?;
            return self.handle_update_failure::<()>(user_id, &mut tx).await;
        }

        self.clear_refresh_tokens(user_id).await?;
        Ok(())
    }
}

impl PostgresUserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn clear_refresh_tokens(&self, user_id: Uuid) -> Result<()> {
        query!("DELETE FROM user_tokens WHERE user_id = $1", user_id)
            .execute(&self.pool)
            .await?;
        Ok(())
    }

    async fn handle_update_failure<T>(
        &self,
        user_id: Uuid,
        transaction: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    ) -> Result<T> {
        let exists = query_scalar!(
            "SELECT EXISTS(SELECT 1 FROM users WHERE id = $1)",
            user_id
        )
        .fetch_one(&mut **transaction)
        .await?
        .unwrap_or(false);

        if exists {
            Err(UserError::Conflict(user_id.to_string()))
        } else {
            Err(UserError::UserNotFound)
        }
    }
}
