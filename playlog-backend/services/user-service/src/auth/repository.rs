use super::{AuthError, RegisterRequest, RegisterResponse, Result, User};
use crate::shared::AccountStatus;
use async_trait::async_trait;
use chrono::{DateTime, Utc};
use jwt_common::Role;
use sqlx::{query, query_as, query_scalar, FromRow, PgPool, Postgres, Transaction};
use uuid::Uuid;

#[async_trait]
pub trait AuthRepository: Send + Sync {
    async fn find_by_email_or_username(&self, identifier: &str) -> Result<Option<User>>;
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
    async fn email_exists(&self, email: &str) -> Result<bool>;
    async fn username_exists(&self, username: &str) -> Result<bool>;
    async fn create_user(
        &self,
        request: &RegisterRequest,
        is_admin: bool,
    ) -> Result<RegisterResponse>;
    async fn get_admin_count(&self) -> Result<i64>;
    async fn get_user_role(&self, id: Uuid) -> Result<Role>;
    async fn is_token_valid(&self, token: &str) -> Result<bool>;
    async fn save_refresh_token(
        &self,
        user_id: Uuid,
        token: &str,
        expiration_date: &DateTime<Utc>,
    ) -> Result<()>;
    async fn revoke_token(&self, token: &str) -> Result<bool>;
    async fn clear_expired_tokens(&self) -> Result<u64>;
}

#[derive(Debug, Clone)]
pub struct PostgresAuthRepository {
    pool: PgPool,
}

#[async_trait]
impl AuthRepository for PostgresAuthRepository {
    async fn find_by_email_or_username(&self, identifier: &str) -> Result<Option<User>> {
        let user = query_as!(
            User,
            r#"
                SELECT id, username, email, password, account_status AS "account_status!: AccountStatus"
                FROM users
                WHERE email = $1 OR username = $1
            "#,
            identifier
        )
            .fetch_optional(&self.pool)
            .await?;
        Ok(user)
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let user = query_as!(
            User,
            r#"
                SELECT id, username, email, password, account_status AS "account_status!: AccountStatus"
                FROM users
                WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await?;
        Ok(user)
    }

    async fn email_exists(&self, email: &str) -> Result<bool> {
        let exists = query_scalar!("SELECT 1 FROM users WHERE email = $1", email)
            .fetch_optional(&self.pool)
            .await?
            .is_some();
        Ok(exists)
    }

    async fn username_exists(&self, username: &str) -> Result<bool> {
        let exists = query_scalar!("SELECT 1 FROM users WHERE username = $1", username)
            .fetch_optional(&self.pool)
            .await?
            .is_some();
        Ok(exists)
    }

    async fn create_user(
        &self,
        request: &RegisterRequest,
        is_admin: bool,
    ) -> Result<RegisterResponse> {
        let mut transaction = self.pool.begin().await?;

        let user = self.insert_user(&mut transaction, request).await?;
        let created_at = self
            .insert_profile(&mut transaction, user.id, request)
            .await?;
        self.assign_role(&mut transaction, user.id, is_admin)
            .await?;

        transaction.commit().await?;
        Ok(RegisterResponse::new(
            user.id,
            user.username,
            user.email,
            created_at,
        ))
    }

    async fn get_admin_count(&self) -> Result<i64> {
        let result = query_scalar!(
            r#"
                SELECT COUNT(1)
                FROM users u
                         JOIN user_roles ur ON ur.user_id = u.id
                         JOIN roles r ON r.id = ur.role_id
                WHERE r.name = 'ADMIN'
            "#
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(0);
        Ok(result)
    }

    async fn get_user_role(&self, user_id: Uuid) -> Result<Role> {
        let role_name = query_scalar!(
            r#"
                SELECT r.name
                FROM user_roles ur
                    INNER JOIN roles r ON ur.role_id = r.id
                    INNER JOIN users u ON ur.user_id = u.id
                WHERE user_id = $1 and u.account_status = 'ACTIVE'
            "#,
            user_id
        )
        .fetch_one(&self.pool)
        .await?;
        Ok(role_name.parse().map_err(|_| AuthError::UserNotFound)?)
    }

    async fn is_token_valid(&self, token: &str) -> Result<bool> {
        let exists = query_scalar!(
            r#"
                SELECT EXISTS (
                    SELECT 1
                    FROM user_tokens
                    WHERE token = $1 AND revoked = false AND expires_at > now()
                )
            "#,
            token
        )
        .fetch_one(&self.pool)
        .await?
        .unwrap_or(false);
        Ok(exists)
    }

    async fn save_refresh_token(
        &self,
        user_id: Uuid,
        token: &str,
        expiration_date: &DateTime<Utc>,
    ) -> Result<()> {
        query!(
            r#"
                INSERT INTO user_tokens (user_id, token, expires_at)
                VALUES ($1, $2, $3)
            "#,
            user_id,
            token,
            expiration_date
        )
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn revoke_token(&self, token: &str) -> Result<bool> {
        let revoked = query_scalar!(
            r#"
                UPDATE user_tokens
                SET revoked = true
                WHERE token = $1
                RETURNING 1
            "#,
            token
        )
        .fetch_optional(&self.pool)
        .await?
        .is_some();
        Ok(revoked)
    }

    async fn clear_expired_tokens(&self) -> Result<u64> {
        let result = query!("DELETE FROM user_tokens WHERE expires_at < now()")
            .execute(&self.pool)
            .await?;
        Ok(result.rows_affected())
    }
}

impl PostgresAuthRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn insert_user(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        request: &RegisterRequest,
    ) -> Result<InsertedUser> {
        let user = query_as!(
            InsertedUser,
            r#"
                INSERT INTO users (username, email, password)
                VALUES ($1, $2, $3)
                RETURNING id, username, email
            "#,
            &request.username,
            &request.email,
            &request.password
        )
        .fetch_one(&mut **transaction)
        .await?;
        Ok(user)
    }

    async fn insert_profile(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
        request: &RegisterRequest,
    ) -> Result<DateTime<Utc>> {
        let result = query_scalar!(
            r#"
                INSERT INTO user_profiles (user_id, first_name, last_name, birthdate)
                VALUES ($1, $2, $3, $4)
                RETURNING created_at
            "#,
            user_id,
            request.first_name,
            request.last_name,
            request.birthdate
        )
        .fetch_one(&mut **transaction)
        .await?;
        Ok(result)
    }

    async fn assign_role(
        &self,
        transaction: &mut Transaction<'_, Postgres>,
        user_id: Uuid,
        is_admin: bool,
    ) -> Result<()> {
        let role = if is_admin { "ADMIN" } else { "USER" };
        query!(
            r#"
                INSERT INTO user_roles (user_id, role_id)
                VALUES ($1, (SELECT id FROM roles WHERE name = $2))
            "#,
            user_id,
            role
        )
        .execute(&mut **transaction)
        .await?;
        Ok(())
    }
}

#[derive(FromRow)]
struct InsertedUser {
    id: Uuid,
    username: String,
    email: String,
}
