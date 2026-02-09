use super::{
    create_tokens, hash_password, verify_password, AccountStatus, AuthError, AuthRepository,
    LoginRequest, RegisterRequest, RegisterResponse, Result, Tokens,
};
use crate::config::{AdminBootstrapConfig, AppConfig};
use uuid::Uuid;

pub struct AuthService {
    repository: Box<dyn AuthRepository>,
}

impl AuthService {
    pub fn new(repository: Box<dyn AuthRepository>) -> Self {
        Self { repository }
    }

    pub async fn login(&self, request: LoginRequest, config: &AppConfig) -> Result<Tokens> {
        let user = self
            .repository
            .find_by_email_or_username(&request.identifier)
            .await?;

        if matches!(user.account_status, AccountStatus::Blocked) {
            return Err(AuthError::UserBlocked);
        }
        if matches!(user.account_status, AccountStatus::Deactivated) {
            return Err(AuthError::UserNotFound);
        }

        verify_password(&request.password, &user.password)?;

        let tokens = self.generate_tokens(config, user.id).await?;
        Ok(tokens)
    }

    pub async fn register(&self, request: RegisterRequest) -> Result<RegisterResponse> {
        if self.email_exists(&request.email).await? {
            return Err(AuthError::EmailAlreadyExists);
        }
        if self.username_exists(&request.username).await? {
            return Err(AuthError::UsernameTaken);
        }

        let password = hash_password(&request.password)?;
        let request = RegisterRequest {
            password,
            ..request
        };

        self.repository.create_user(&request, false).await
    }

    pub async fn revoke_token(&self, token: &str) -> Result<bool> {
        self.repository.revoke_token(token).await
    }

    pub async fn refresh_token(
        &self,
        config: &AppConfig,
        token: &str,
        user_id: Uuid,
    ) -> Result<Tokens> {
        self.revoke_token(token).await?;
        let tokens = self.generate_tokens(config, user_id).await?;
        Ok(tokens)
    }

    pub async fn ensure_admin(&self, config: Option<AdminBootstrapConfig>) -> Result<()> {
        if let Some(admin_config) = config {
            let admin_created = self.init_admin(admin_config).await?;
            if admin_created {
                tracing::info!("Admin account created successfully");
            }
        }
        Ok(())
    }

    async fn username_exists(&self, username: &str) -> Result<bool> {
        self.repository.username_exists(username).await
    }

    async fn email_exists(&self, email: &str) -> Result<bool> {
        self.repository.email_exists(email).await
    }

    async fn generate_tokens(&self, config: &AppConfig, user_id: Uuid) -> Result<Tokens> {
        let role = self.repository.get_user_role(user_id).await?;
        let (tokens, refresh_expiration) = create_tokens(config, user_id, role)?;

        self.repository
            .save_refresh_token(user_id, &tokens.1, &refresh_expiration)
            .await?;

        Ok(tokens)
    }

    async fn init_admin(&self, config: AdminBootstrapConfig) -> Result<bool> {
        if self.email_exists(&config.email).await? {
            return Ok(false);
        }

        if self.username_exists(&config.username).await? {
            return Ok(false);
        }

        let password = hash_password(&config.temp_password)?;

        let request = RegisterRequest::new(config.username, config.email, password);

        let result = self.repository.create_user(&request, true).await;
        Ok(result.is_ok())
    }
}
