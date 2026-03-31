use application::repository::{
    session::SessionRepositoryImpl, user::UserRepositoryImpl,
};
use infrastructure::persistence::{
    postgres::{POSTGRES_MIGRATOR, repository::PostgresRepositoryImpl},
    redis::repository::RedisRepositoryImpl,
};
use lib::{
    application::impl_has,
    bootstrap::impl_repositories,
    infrastructure::persistence::mobc_sqlx::MigratorExt as _,
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
    #[expect(dead_code, reason = "we may use config in the future")]
    config: RepositoriesConfig,
    postgres: Pool<SqlxConnectionManager<Postgres>>,
    redis: Pool<RedisConnectionManager>,
}

impl RepositoriesModule {
    pub(crate) async fn new(config: &RepositoriesConfig) -> Self {
        Self {
            config: config.clone(),
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

impl_has! {
    struct: Modules,
    Pool<SqlxConnectionManager<Postgres>>: |s| &s.repositories.postgres,
    Pool<RedisConnectionManager>: |s| &s.repositories.redis,
}

impl_repositories! {
    struct: Modules,
    UserRepositoryImpl: |_s| &PostgresRepositoryImpl,
    SessionRepositoryImpl: |_s| &RedisRepositoryImpl,
}
