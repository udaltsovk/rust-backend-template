use application::repository::RepositoriesModuleExt;
use domain::{session::Session, user::User};
use infrastructure::persistence::{
    postgres::{POSTGRES_MIGRATOR, repository::PostgresRepositoryImpl},
    redis::repository::RedisRepositoryImpl,
};
use lib::{
    infrastructure::persistence::{
        mobc_sqlx::MigratorExt as _, postgres::error::PostgresAdapterError,
        redis::error::RedisAdapterError,
    },
    mobc_redis::{RedisConnectionManager, redis},
    mobc_sqlx::{
        SqlxConnectionManager,
        mobc::Pool,
        sqlx::{Postgres, postgres::PgConnectOptions},
    },
    tap::Pipe as _,
};

use crate::modules::repositories::config::RedisConfig;
pub use crate::modules::repositories::config::{
    PostgresConfig, RepositoriesConfig,
};

mod config;

#[derive(Clone)]
pub struct RepositoriesModule {
    user_repository: PostgresRepositoryImpl<User>,
    session_repository: RedisRepositoryImpl<Session>,
}

impl RepositoriesModule {
    pub(crate) async fn new(config: &RepositoriesConfig) -> Self {
        let postgres = Self::setup_postgres(&config.postgres).await;
        let redis = Self::setup_redis(&config.redis);

        let user_repository = PostgresRepositoryImpl::new(&postgres);
        let session_repository = RedisRepositoryImpl::new(&redis);

        Self {
            user_repository,
            session_repository,
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

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error(transparent)]
    Postgres(#[from] PostgresAdapterError),
    #[error(transparent)]
    Redis(#[from] RedisAdapterError),
}

impl RepositoriesModuleExt for RepositoriesModule {
    type Error = RepositoryError;
    type SessionRepo = RedisRepositoryImpl<Session>;
    type UserRepo = PostgresRepositoryImpl<User>;

    fn user_repository(&self) -> &Self::UserRepo {
        &self.user_repository
    }

    fn session_repository(&self) -> &Self::SessionRepo {
        &self.session_repository
    }
}
