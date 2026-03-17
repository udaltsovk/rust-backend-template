use crate::service::{secret_hasher::SecretHasherService, token::TokenService};

pub mod secret_hasher;
pub mod token;

pub trait Services: SecretHasherService + TokenService {}

impl<T> Services for T where T: SecretHasherService + TokenService {}
