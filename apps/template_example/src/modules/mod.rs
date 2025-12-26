use application::usecase::{UseCase, client::ClientUseCase};
use domain::client::Client;
use infrastructure::persistence::postgres::POSTGRES_MIGRATOR;
use lib::{
    infrastructure::persistence::mobc_sqlx::MigratorExt as _,
    mobc_sqlx::{
        SqlxConnectionManager, mobc::Pool, sqlx::postgres::PgConnectOptions,
    },
    tap::Pipe as _,
};
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
        let postgres = PgConnectOptions::new()
            .username(&config::POSTGRES_USER)
            .password(&config::POSTGRES_PASSWORD)
            .host(&config::POSTGRES_HOST)
            .port(*config::POSTGRES_PORT)
            .database(&config::POSTGRES_DATABASE)
            .pipe(SqlxConnectionManager::new)
            .pipe(Pool::new);

        POSTGRES_MIGRATOR.migrate(&postgres).await;

        let repositories_module = RepositoriesModule::new(&postgres);
        let services_module = ServicesModule::new(&config::JWT_SECRET);

        Self {
            client_usecase: UseCase::new(repositories_module, services_module),
        }
    }
}
