use crate::auth::AuthRepository;
use std::time::Duration;
use tokio::time::sleep;

pub fn schedule_token_cleanup(repository: Box<dyn AuthRepository>) {
    tokio::spawn(async move {
        loop {
            match repository.clear_expired_tokens().await {
                Ok(deleted) => tracing::info!("Cleared {} expired tokens", deleted),
                Err(err) => tracing::warn!("Failed to clear expired tokens: {err:?}"),
            }
            sleep(Duration::from_secs(86_400)).await; // 1 day
        }
    });
}
