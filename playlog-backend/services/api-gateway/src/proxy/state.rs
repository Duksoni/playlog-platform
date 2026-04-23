use crate::proxy::ProxyClient;
use jwt_common::JwtConfig;

pub struct ServiceAppState {
    pub service_url: String,
    pub proxy_client: ProxyClient,
    pub jwt_config: JwtConfig,
}

impl ServiceAppState {
    pub fn new(
        service_url: String,
        proxy_client: ProxyClient,
        jwt_config: JwtConfig,
    ) -> Self {
        Self {
            service_url,
            proxy_client,
            jwt_config,
        }
    }
}
