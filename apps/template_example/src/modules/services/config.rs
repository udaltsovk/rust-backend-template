use fromenv::FromEnv;
use infrastructure::services::token::jwt::{
    DecodingKey, EncodingKey, JwtService,
};

#[derive(FromEnv, Clone)]
pub struct ServicesConfig {
    #[env(nested)]
    pub jwt: JwtConfig,
}

#[derive(FromEnv, Clone)]
#[env(prefix = "JWT_")]
pub struct JwtConfig {
    pub secret: String,
}

impl From<&JwtConfig> for JwtService {
    fn from(config: &JwtConfig) -> Self {
        let secret = config.secret.as_bytes();
        Self::new(
            EncodingKey::from_secret(secret),
            DecodingKey::from_secret(secret),
        )
    }
}
