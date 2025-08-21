use std::sync::Arc;

use infrastructure_persistence_postgres::Postgres;
use kernel::{application::usecase::UseCase, domain::client::Client};
use presentation_api_rest::module::ModulesExt;

use crate::{
    config,
    modules::{repositories::RepositoriesModule, services::ServicesModule},
};

mod repositories;
mod services;

#[derive(Clone)]
pub struct Modules {
    client_usecase: Arc<UseCase<RepositoriesModule, ServicesModule, Client>>,
}
impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;
    type ServicesModule = ServicesModule;

    fn client_usecase(
        &self,
    ) -> &UseCase<Self::RepositoriesModule, Self::ServicesModule, Client> {
        &self.client_usecase
    }
}
impl Modules {
    pub async fn new() -> Self {
        let postgres_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            *config::POSTGRES_USER,
            *config::POSTGRES_PASSWORD,
            *config::POSTGRES_ADDRESS,
            *config::POSTGRES_PORT,
            *config::POSTGRES_NAME,
        );
        let postgres = Postgres::new(&postgres_url).await;

        let repositories_module = RepositoriesModule::new(&postgres);
        let services_module = ServicesModule::new(&config::JWT_SECRET);

        let client_usecase = Arc::new(UseCase::new(
            repositories_module.clone(),
            services_module.clone(),
        ));

        Self {
            client_usecase,
        }
    }
}
