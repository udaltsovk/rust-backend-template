use lib::{
    infrastructure::persistence::redis::RedisPool,
    mobc_redis::RedisConnectionManager,
    mobc_sqlx::mobc::Pool, tap::Pipe as _,
};

pub(super) use self::config::RedisConfig;
use super::RepositoriesModule;

mod config;

impl RepositoriesModule {
    pub(super) fn setup_redis(
        config: &RedisConfig,
    ) -> RedisPool {
        redis::Client::from(config)
            .pipe(RedisConnectionManager::new)
            .pipe(Pool::new)
    }
}
