use super::{Result, SimpleUser, UpdateProfileRequest, UserDetails, UserError};
use crate::shared::AccountStatus;
use async_trait::async_trait;
use jwt_common::Role;
use sqlx::{postgres::PgArguments, query, query::Query, query_as, query_scalar, PgPool, Postgres};
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
    async fn update_password(&self, user_id: Uuid, new_password: &str) -> Result<bool>;
    async fn update_user_role(&self, user_id: Uuid, new_role: Role) -> Result<()>;
    async fn deactivate_account(&self, user_id: Uuid) -> Result<()>;
    async fn block_user(&self, user_id: Uuid) -> Result<()>;
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
                SELECT u.id, username
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
                SELECT u.id, username, r.name as role, first_name, last_name, birthdate, created_at
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
        let rows_changed = query!(
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
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(rows_changed == 1)
    }

    async fn update_password(&self, user_id: Uuid, new_password: &str) -> Result<bool> {
        let rows_changed = query!(
            r#"
                UPDATE users
                SET password = $2
                WHERE id = $1
            "#,
            user_id,
            new_password,
        )
        .execute(&self.pool)
        .await?
        .rows_affected();
        Ok(rows_changed == 1)
    }

    async fn update_user_role(&self, user_id: Uuid, new_role: Role) -> Result<()> {
        let prepared_query = query!(
            r#"
                UPDATE user_roles
                SET role_id = (SELECT id FROM roles WHERE name = $2)
                WHERE user_id = $1
            "#,
            user_id,
            new_role.as_db_value()
        );
        self.execute_user_state_update(user_id, prepared_query)
            .await?;
        Ok(())
    }

    async fn deactivate_account(&self, user_id: Uuid) -> Result<()> {
        let prepared_query = query!(
            "UPDATE users SET account_status = 'DEACTIVATED' WHERE id = $1",
            user_id
        );
        Ok(self
            .execute_user_state_update(user_id, prepared_query)
            .await?)
    }

    async fn block_user(&self, user_id: Uuid) -> Result<()> {
        let prepared_query = query!(
            "UPDATE users SET account_status = 'BLOCKED' WHERE id = $1",
            user_id
        );
        self.execute_user_state_update(user_id, prepared_query)
            .await?;
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

    async fn execute_user_state_update(
        &self,
        user_id: Uuid,
        prepared_query: Query<'_, Postgres, PgArguments>,
    ) -> Result<()> {
        let rows_changed = prepared_query.execute(&self.pool).await?.rows_affected();
        if rows_changed == 1 {
            self.clear_refresh_tokens(user_id).await?;
        }
        Ok(())
    }
}
