use std::sync::Arc;

use application::service::ServicesModuleExt;
use infrastructure::services::token::jwt::{JwtAdapterError, JwtService};

#[derive(Clone)]
pub struct ServicesModule {
    token_service: Arc<JwtService>,
}

impl ServicesModule {
    pub(crate) fn new(jwt_secret: &str) -> Self {
        let token_service = Arc::new(JwtService::new(jwt_secret));
        Self {
            token_service,
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

    fn token_service(&self) -> Arc<Self::TokenService> {
        self.token_service.clone()
    }
}
