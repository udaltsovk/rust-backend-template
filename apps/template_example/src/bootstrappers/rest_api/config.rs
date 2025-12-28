use std::net::{IpAddr, SocketAddr};

use better_config::{EnvConfig, env};

#[env(EnvConfig(prefix = "SERVER_"))]
pub struct RestApiConfig {
    #[conf(default = "::")]
    pub host: IpAddr,
    #[conf(default = "8080")]
    pub port: u16,
    #[conf(default = "localhost")]
    pub domain: String,
}

impl From<&RestApiConfig> for SocketAddr {
    fn from(config: &RestApiConfig) -> Self {
        Self::new(config.host, config.port)
    }
}
