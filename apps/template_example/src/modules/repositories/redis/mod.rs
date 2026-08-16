use lib::{
    infrastructure::persistence::RedisPool, mobc_redis::RedisConnectionManager,
    tap::Pipe as _,
};

pub(super) use self::config::RedisConfig;
use super::RepositoriesModule;

mod config;

impl RepositoriesModule {
    pub(super) fn setup_redis(config: &RedisConfig) -> RedisPool {
        redis::Client::from(config)
            .pipe(RedisConnectionManager::new)
            .pipe(RedisPool::new)
    }
}
