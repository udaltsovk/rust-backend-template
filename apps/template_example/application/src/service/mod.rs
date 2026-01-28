use crate::service::{hasher::HasherService, token::TokenService};

pub mod hasher;
pub mod token;

pub trait ServicesModuleExt: Send + Sync {
    fn password_hasher_service(&self) -> &dyn HasherService;

    fn token_service(&self) -> &dyn TokenService;
}
