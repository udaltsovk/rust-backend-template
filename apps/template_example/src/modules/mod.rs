pub use self::config::ModulesConfig;
use self::{
    repositories::RepositoriesModule,
    services::ServicesModule,
};

mod config;
mod repositories;
mod services;

#[derive(Clone)]
pub struct Modules {
    config: &'static ModulesConfig,
    repositories: RepositoriesModule,
    services: ServicesModule,
}

impl Modules {
    pub async fn init(
        config: &'static ModulesConfig,
    ) -> Self {
        Self {
            config,
            repositories: RepositoriesModule::new(
                &config.repositories,
            )
            .await,
            services: ServicesModule::new(&config.services),
        }
    }
}
