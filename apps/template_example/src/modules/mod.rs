pub use crate::modules::config::ModulesConfig;
use crate::modules::{
    repositories::RepositoriesModule, services::ServicesModule,
};

mod config;
mod repositories;
mod services;

#[derive(Clone)]
pub struct Modules {
    config: ModulesConfig,
    repositories: RepositoriesModule,
    services: ServicesModule,
}

impl Modules {
    pub async fn init(config: &ModulesConfig) -> Self {
        Self {
            config: config.clone(),
            repositories: RepositoriesModule::new(&config.repositories).await,
            services: ServicesModule::new(&config.services),
        }
    }
}
