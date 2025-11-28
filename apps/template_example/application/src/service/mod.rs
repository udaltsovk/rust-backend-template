use std::fmt::{Debug, Display};

use crate::service::token::TokenService;

pub mod hasher;
pub mod token;

pub trait ServicesModuleExt: Clone + Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::TokenService as TokenService>::AdapterError>;

    type TokenService: TokenService + Send + Sync;

    fn token_service(&self) -> &Self::TokenService;
}
