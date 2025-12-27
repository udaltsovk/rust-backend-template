use std::sync::Arc;

use application::service::ServicesModuleExt;
use infrastructure::services::token::jwt::{
    DecodingKey, EncodingKey, JwtAdapterError, JwtService,
};

use crate::config;

#[derive(Clone)]
pub struct ServicesModule {
    token_service: Arc<JwtService>,
}

impl ServicesModule {
    pub(crate) fn new() -> Self {
        Self {
            token_service: Self::jwt_service(),
        }
    }

    fn jwt_service() -> Arc<JwtService> {
        let secret = config::JWT_SECRET.as_bytes();
        Arc::new(JwtService::new(
            EncodingKey::from_secret(secret),
            DecodingKey::from_secret(secret),
        ))
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("JWT service error: {0}")]
    Jwt(#[from] JwtAdapterError),
}

impl ServicesModuleExt for ServicesModule {
    type Error = ServiceError;
    type TokenService = JwtService;

    fn token_service(&self) -> &Self::TokenService {
        &self.token_service
    }
}
