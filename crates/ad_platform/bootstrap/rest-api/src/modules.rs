use std::sync::Arc;

use infrastructure_persistence_postgres::{Db, module::RepositoriesModule};
use kernel::{application::usecase::UseCase, domain::client::Client};
use presentation_api_rest::module::ModulesExt;

use crate::config;

#[derive(Clone)]
pub struct Modules {
    client_usecase: Arc<UseCase<RepositoriesModule, Client>>,
}
impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;

    fn client_usecase(&self) -> &UseCase<Self::RepositoriesModule, Client> {
        &self.client_usecase
    }
}
impl Modules {
    pub async fn new() -> Self {
        let database_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            *config::DB_USER,
            *config::DB_PASSWORD,
            *config::DB_ADDRESS,
            *config::DB_PORT,
            *config::DB_NAME,
        );
        let db = Db::new(&database_url).await;

        let repositories_module = RepositoriesModule::new(&db);

        let client_usecase =
            Arc::new(UseCase::new(repositories_module.clone()));

        Self {
            client_usecase,
        }
    }
}
