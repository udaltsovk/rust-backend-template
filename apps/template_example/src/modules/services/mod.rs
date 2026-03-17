use application::service::{
    secret_hasher::SecretHasherServiceImpl, token::TokenServiceImpl,
};
use infrastructure::services::{
    hasher::argon2::{Argon2Service, HasArgon2Service},
    token::jwt::{HasJwtService, JwtService},
};

use crate::Modules;
pub use crate::modules::services::config::ServicesConfig;

mod config;

#[derive(Clone)]
pub struct ServicesModule {
    password_hasher_service: Argon2Service,
    token_service: JwtService,
}

impl ServicesModule {
    pub(crate) fn new(config: &ServicesConfig) -> Self {
        Self {
            password_hasher_service: Argon2Service::new(),
            token_service: JwtService::from(&config.jwt),
        }
    }
}

impl HasArgon2Service for Modules {
    fn argon2_service(&self) -> &Argon2Service {
        &self.services.password_hasher_service
    }
}

impl AsRef<dyn SecretHasherServiceImpl<Self>> for Modules {
    fn as_ref(&self) -> &dyn SecretHasherServiceImpl<Self> {
        &self.services.password_hasher_service
    }
}

impl HasJwtService for Modules {
    fn jwt_service(&self) -> &JwtService {
        &self.services.token_service
    }
}

impl AsRef<dyn TokenServiceImpl<Self>> for Modules {
    fn as_ref(&self) -> &dyn TokenServiceImpl<Self> {
        &self.services.token_service
    }
}
