use crate::proxy::ProxyClient;
use jwt_common::JwtConfig;

pub struct UserAppState {
    pub user_service_url: String,
    pub proxy_client: ProxyClient,
    pub jwt_config: JwtConfig,
}

impl UserAppState {
    pub fn new(user_service_url: String, proxy_client: ProxyClient, jwt_config: JwtConfig) -> Self {
        Self {
            user_service_url,
            proxy_client,
            jwt_config,
        }
    }
}
