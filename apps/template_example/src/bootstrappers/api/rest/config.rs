use std::net::{IpAddr, SocketAddr};

use fromenv::FromEnv;

#[derive(FromEnv)]
#[env(prefix = "SERVER_")]
pub struct RestApiConfig {
    #[env(default = "::")]
    pub host: IpAddr,
    #[env(default = "8080")]
    pub port: u16,
    pub domain: String,
    #[env(default = "30000")]
    pub request_timeout_ms: u64,
    #[env(default = "1048576")]
    pub body_limit_bytes: usize,
}

impl From<&RestApiConfig> for SocketAddr {
    fn from(config: &RestApiConfig) -> Self {
        Self::new(config.host, config.port)
    }
}
