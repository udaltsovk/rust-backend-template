use std::sync::Arc;

use better_config::{EnvConfig, env};
use infrastructure::services::token::jwt::{
    DecodingKey, EncodingKey, JwtService,
};

#[env(EnvConfig)]
pub struct ServicesConfig {
    #[env]
    pub jwt: JwtConfig,
}

#[env(EnvConfig(prefix = "JWT_"))]
pub struct JwtConfig {
    #[conf(default = "changeme")]
    pub secret: String,
}

impl From<&JwtConfig> for Arc<JwtService> {
    fn from(config: &JwtConfig) -> Self {
        let secret = config.secret.as_bytes();
        Self::new(JwtService::new(
            EncodingKey::from_secret(secret),
            DecodingKey::from_secret(secret),
        ))
    }
}
