use application::repository::RepositoriesModuleExt;
use domain::client::Client;
use infrastructure::persistence::postgres::{
    POSTGRES_MIGRATOR, repository::PostgresRepositoryImpl,
};
use lib::{
    infrastructure::persistence::{
        mobc_sqlx::MigratorExt as _, postgres::error::PostgresAdapterError,
    },
    mobc_sqlx::{
        SqlxConnectionManager,
        mobc::Pool,
        sqlx::{Postgres, postgres::PgConnectOptions},
    },
    tap::Pipe as _,
};

use crate::config;

#[derive(Clone)]
pub struct RepositoriesModule {
    client_repository: PostgresRepositoryImpl<Client>,
}

impl RepositoriesModule {
    pub(crate) async fn new() -> Self {
        let postgres = Self::setup_postgres().await;

        let client_repository = PostgresRepositoryImpl::new(&postgres);

        Self {
            client_repository,
        }
    }

    async fn setup_postgres() -> Pool<SqlxConnectionManager<Postgres>> {
        let postgres = PgConnectOptions::new()
            .username(&config::POSTGRES_USER)
            .password(&config::POSTGRES_PASSWORD)
            .host(&config::POSTGRES_HOST)
            .port(*config::POSTGRES_PORT)
            .database(&config::POSTGRES_DATABASE)
            .pipe(SqlxConnectionManager::new)
            .pipe(Pool::new);

        POSTGRES_MIGRATOR.migrate(&postgres).await;

        postgres
    }
}

#[derive(thiserror::Error, Debug)]
pub enum RepositoryError {
    #[error(transparent)]
    Postgres(#[from] PostgresAdapterError),
}

impl RepositoriesModuleExt for RepositoriesModule {
    type ClientRepo = PostgresRepositoryImpl<Client>;
    type Error = RepositoryError;

    fn client_repository(&self) -> &Self::ClientRepo {
        &self.client_repository
    }
}
