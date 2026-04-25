pub mod secret_hasher;
pub mod token;

pub trait AuthServices = secret_hasher::SecretHasherService
    + token::TokenService;
