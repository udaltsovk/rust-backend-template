use application::service::{
    secret_hasher::SecretHasherServiceImpl, token::TokenServiceImpl,
};
use infrastructure::services::{
    hasher::argon2::Argon2Service, token::jwt::JwtService,
};
use lib::{application::impl_Has, bootstrap::impl_services};

use crate::Modules;
pub use crate::modules::services::config::ServicesConfig;

mod config;

#[derive(Clone)]
pub struct ServicesModule {
    #[expect(dead_code, reason = "we may use config in the future")]
    config: ServicesConfig,
    password_hasher_service: Argon2Service,
    token_service: JwtService,
}

impl ServicesModule {
    pub(crate) fn new(config: &ServicesConfig) -> Self {
        Self {
            config: config.clone(),
            password_hasher_service: Argon2Service::new(),
            token_service: JwtService::from(&config.jwt),
        }
    }
}

impl_Has! {
    struct: Modules,
    Argon2Service: |s| &s.services.password_hasher_service,
    JwtService: |s| &s.services.token_service,
}

impl_services! {
    struct: Modules,
    SecretHasherServiceImpl: |s| &s.services.password_hasher_service,
    TokenServiceImpl: |s| &s.services.token_service,
}
