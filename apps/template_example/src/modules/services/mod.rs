use lib::{
    application::impl_has, bootstrap::impl_services,
};

pub use self::config::ServicesConfig;
use super::Modules;
use crate::features::user_auth::{
    application::service::{
        secret_hasher::DelegateSecretHasherService,
        token::DelegateTokenService,
    },
    infrastructure::services::{
        hasher::argon2::Argon2Service,
        token::jwt::JwtService,
    },
};

mod config;

#[derive(Clone)]
pub struct ServicesModule {
    password_hasher: Argon2Service,
    token: JwtService,
}

impl ServicesModule {
    pub(crate) fn new(config: &ServicesConfig) -> Self {
        Self {
            password_hasher: Argon2Service::new(),
            token: JwtService::from(&config.jwt),
        }
    }
}

impl_has! {
    struct: Modules,
    Argon2Service: |s| &s.services.password_hasher,
    JwtService: |s| &s.services.token,
}

impl_services! {
    struct: Modules,
    DelegateSecretHasherService: Argon2Service,
    DelegateTokenService: JwtService,
}
