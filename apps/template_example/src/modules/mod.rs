pub use crate::modules::config::ModulesConfig;
use crate::modules::{
    repositories::RepositoriesModule, services::ServicesModule,
};

mod config;
mod repositories;
mod services;

#[derive(Clone)]
pub struct Modules {
    repositories: RepositoriesModule,
    services: ServicesModule,
}

impl Modules {
    pub async fn init(config: &ModulesConfig) -> Self {
        Self {
            repositories: RepositoriesModule::new(&config.repositories).await,
            services: ServicesModule::new(&config.services),
        }
    }
}
