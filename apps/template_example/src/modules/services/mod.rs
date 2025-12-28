use std::sync::Arc;

use application::service::ServicesModuleExt;
use infrastructure::services::token::jwt::{JwtAdapterError, JwtService};

pub use crate::modules::services::config::ServicesConfig;

mod config;

#[derive(Clone)]
pub struct ServicesModule {
    token_service: Arc<JwtService>,
}

impl ServicesModule {
    pub(crate) fn new(config: &ServicesConfig) -> Self {
        Self {
            token_service: Arc::from(&config.jwt),
        }
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
