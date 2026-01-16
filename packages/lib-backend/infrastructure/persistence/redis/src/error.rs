use mobc_redis::{mobc::Error as MobcError, redis::RedisError};

#[derive(thiserror::Error, Debug)]
pub enum RedisAdapterError {
    #[error(transparent)]
    Pool(#[from] MobcError<RedisError>),

    #[error(transparent)]
    Database(#[from] RedisError),
}
