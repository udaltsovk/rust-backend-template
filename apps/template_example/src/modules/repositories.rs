use application::repository::RepositoriesModuleExt;
use domain::client::Client;
use infrastructure::persistence::postgres::repository::PostgresRepositoryImpl;
use lib::infrastructure::persistence::postgres::error::PostgresAdapterError;
use mobc_sqlx::{SqlxConnectionManager, mobc::Pool, sqlx::Postgres};

#[derive(Clone)]
pub struct RepositoriesModule {
    client_repository: PostgresRepositoryImpl<Client>,
}

impl RepositoriesModule {
    pub fn new(postgres: &Pool<SqlxConnectionManager<Postgres>>) -> Self {
        let client_repository = PostgresRepositoryImpl::new(postgres);

        Self {
            client_repository,
        }
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
