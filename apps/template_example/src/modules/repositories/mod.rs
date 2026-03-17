use application::repository::{
    session::SessionRepositoryImpl, user::UserRepositoryImpl,
};
use infrastructure::persistence::{
    postgres::{POSTGRES_MIGRATOR, repository::PostgresRepositoryImpl},
    redis::repository::RedisRepositoryImpl,
};
use lib::{
    infrastructure::persistence::{HasPool, mobc_sqlx::MigratorExt as _},
    mobc_redis::{RedisConnectionManager, redis},
    mobc_sqlx::{
        SqlxConnectionManager,
        mobc::Pool,
        sqlx::{Postgres, postgres::PgConnectOptions},
    },
    tap::Pipe as _,
};

pub use crate::modules::repositories::config::{
    PostgresConfig, RepositoriesConfig,
};
use crate::{Modules, modules::repositories::config::RedisConfig};

mod config;

#[derive(Clone)]
pub struct RepositoriesModule {
    postgres: Pool<SqlxConnectionManager<Postgres>>,
    redis: Pool<RedisConnectionManager>,
}

impl RepositoriesModule {
    pub(crate) async fn new(config: &RepositoriesConfig) -> Self {
        Self {
            postgres: Self::setup_postgres(&config.postgres).await,
            redis: Self::setup_redis(&config.redis),
        }
    }

    async fn setup_postgres(
        config: &PostgresConfig,
    ) -> Pool<SqlxConnectionManager<Postgres>> {
        let postgres = PgConnectOptions::from(config)
            .pipe(SqlxConnectionManager::new)
            .pipe(Pool::new);

        POSTGRES_MIGRATOR.migrate(&postgres).await;

        postgres
    }

    fn setup_redis(config: &RedisConfig) -> Pool<RedisConnectionManager> {
        redis::Client::from(config)
            .pipe(RedisConnectionManager::new)
            .pipe(Pool::new)
    }
}

impl HasPool<SqlxConnectionManager<Postgres>> for Modules {
    fn pool(&self) -> &Pool<SqlxConnectionManager<Postgres>> {
        &self.repositories.postgres
    }
}

impl HasPool<RedisConnectionManager> for Modules {
    fn pool(&self) -> &Pool<RedisConnectionManager> {
        &self.repositories.redis
    }
}

impl AsRef<dyn UserRepositoryImpl<Self> + Sync> for Modules {
    fn as_ref(&self) -> &(dyn UserRepositoryImpl<Self> + Sync) {
        &PostgresRepositoryImpl
    }
}

impl AsRef<dyn SessionRepositoryImpl<Self> + Sync> for Modules {
    fn as_ref(&self) -> &(dyn SessionRepositoryImpl<Self> + Sync) {
        &RedisRepositoryImpl
    }
}
