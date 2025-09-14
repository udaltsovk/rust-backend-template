use std::sync::Arc;

use infrastructure::persistence::postgres::repository::PostgresRepositoryImpl;
use kernel::{
    application::repository::RepositoriesModuleExt, domain::client::Client,
};
use lib::infrastructure::persistence::postgres::Postgres;

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
impl RepositoriesModuleExt for RepositoriesModule {
    type ClientRepo = PostgresRepositoryImpl<Client>;

    fn client_repository(&self) -> Arc<Self::ClientRepo> {
        self.client_repository.clone()
    }
}
