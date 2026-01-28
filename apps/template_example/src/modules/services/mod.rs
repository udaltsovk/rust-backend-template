use application::service::{
    ServicesModuleExt, hasher::HasherService, token::TokenService,
};
use infrastructure::services::{
    hasher::argon2::Argon2Service, token::jwt::JwtService,
};

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

impl ServicesModuleExt for ServicesModule {
    fn password_hasher_service(&self) -> &dyn HasherService {
        &self.password_hasher_service
    }

    fn token_service(&self) -> &dyn TokenService {
        &self.token_service
    }
}
