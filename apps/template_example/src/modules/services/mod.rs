use std::sync::Arc;

use application::service::ServicesModuleExt;
use infrastructure::services::{
    hasher::argon2::{Argon2AdapterError, Argon2Service},
    token::jwt::{JwtAdapterError, JwtService},
};

pub use crate::modules::services::config::ServicesConfig;

mod config;

#[derive(Clone)]
pub struct ServicesModule {
    password_hasher_service: Arc<Argon2Service>,
    token_service: Arc<JwtService>,
}

impl ServicesModule {
    pub(crate) fn new(config: &ServicesConfig) -> Self {
        Self {
            password_hasher_service: Arc::new(Argon2Service::new()),
            token_service: Arc::from(&config.jwt),
        }
    }
}

#[derive(thiserror::Error, Debug)]
pub enum ServiceError {
    #[error("Argon2 service error: {0}")]
    Argon2(#[from] Argon2AdapterError),

    #[error("JWT service error: {0}")]
    Jwt(#[from] JwtAdapterError),
}

impl ServicesModuleExt for ServicesModule {
    type Error = ServiceError;
    type PasswordHasherService = Argon2Service;
    type TokenService = JwtService;

    fn password_hasher_service(&self) -> &Self::PasswordHasherService {
        &self.password_hasher_service
    }

    fn token_service(&self) -> &Self::TokenService {
        &self.token_service
    }
}
