use std::sync::Arc;

use application::repository::RepositoriesModuleExt;
use domain::client::Client;
use infrastructure::persistence::postgres::repository::PostgresRepositoryImpl;
use lib::infrastructure::persistence::postgres::{
    Postgres, error::PostgresAdapterError,
};

#[derive(Clone)]
pub struct RepositoriesModule {
    client_repository: Arc<PostgresRepositoryImpl<Client>>,
}

impl RepositoriesModule {
    pub fn new(postgres: &Postgres) -> Self {
        let client_repository = Arc::new(PostgresRepositoryImpl::new(postgres));

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
