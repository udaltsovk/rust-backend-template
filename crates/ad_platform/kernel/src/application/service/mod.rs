use std::sync::Arc;

use crate::application::service::token::TokenService;

pub mod token;

pub trait ServicesModuleExt: Send + Sync {
    type TokenService: TokenService + Send + Sync;
    fn token_service(&self) -> Arc<Self::TokenService>;
}
