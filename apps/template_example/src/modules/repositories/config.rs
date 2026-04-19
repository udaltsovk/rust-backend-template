use fromenv::FromEnv;

use super::{postgres::PostgresConfig, redis::RedisConfig};

#[derive(FromEnv, Clone)]
pub struct RepositoriesConfig {
    #[env(nested)]
    pub postgres: PostgresConfig,
    #[env(nested)]
    pub redis: RedisConfig,
}
