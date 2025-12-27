use application::usecase::{UseCase, client::ClientUseCase};
use domain::client::Client;
use presentation::api::rest::module::{ModulesExt, UseCaseImpl};

use crate::modules::{
    repositories::RepositoriesModule, services::ServicesModule,
};

mod repositories;
mod services;

#[derive(Clone)]
pub struct Modules {
    #[expect(dead_code, reason = "We might need that in the future")]
    repositories_module: RepositoriesModule,
    #[expect(dead_code, reason = "We might need that in the future")]
    services_module: ServicesModule,
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
        let repositories_module = RepositoriesModule::new().await;
        let services_module = ServicesModule::new();

        let client_usecase =
            UseCase::new(&repositories_module, &services_module);

        Self {
            repositories_module,
            services_module,
            client_usecase,
        }
    }
}
