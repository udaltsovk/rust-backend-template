use application::usecase::{UseCase, client::ClientUseCase};
use domain::client::Client;
use lib::infrastructure::persistence::postgres::Postgres;
use presentation::api::rest::module::{ModulesExt, UseCaseImpl};

use crate::{
    config,
    modules::{repositories::RepositoriesModule, services::ServicesModule},
};

mod repositories;
mod services;

#[derive(Clone)]
pub struct Modules {
    client_usecase: UseCaseImpl<Self, Client>,
}

impl ModulesExt for Modules {
    type RepositoriesModule = RepositoriesModule;
    type ServicesModule = ServicesModule;

    fn client_usecase(
        &self,
    ) -> &impl ClientUseCase<Self::RepositoriesModule, Self::ServicesModule>
    {
        &self.client_usecase
    }
}

impl Modules {
    pub async fn init() -> Self {
        let postgres_url = format!(
            "postgres://{}:{}@{}:{}/{}",
            *config::POSTGRES_USER,
            *config::POSTGRES_PASSWORD,
            *config::POSTGRES_HOST,
            *config::POSTGRES_PORT,
            *config::POSTGRES_DATABASE,
        );
        let postgres = Postgres::new(&postgres_url).await;

        let repositories_module = RepositoriesModule::new(&postgres);
        let services_module = ServicesModule::new(&config::JWT_SECRET);

        Self {
            client_usecase: UseCase::new(repositories_module, services_module),
        }
    }
}
