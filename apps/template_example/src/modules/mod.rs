use application::usecase::{
    UseCase, session::SessionUseCase, user::UserUseCase,
};
use domain::{session::Session, user::User};
use presentation::api::rest::ModulesExt;

pub use crate::modules::config::ModulesConfig;
use crate::modules::{
    repositories::RepositoriesModule, services::ServicesModule,
};

mod config;
mod repositories;
mod services;

#[derive(Clone)]
pub struct Modules {
    #[expect(dead_code, reason = "We might need that in the future")]
    repositories_module: RepositoriesModule,
    #[expect(dead_code, reason = "We might need that in the future")]
    services_module: ServicesModule,
    user_usecase: UseCase<User>,
    session_usecase: UseCase<Session>,
}

impl ModulesExt for Modules {
    fn user_usecase(&self) -> &impl UserUseCase {
        &self.user_usecase
    }

    fn session_usecase(&self) -> &impl SessionUseCase {
        &self.session_usecase
    }
}

impl Modules {
    pub async fn init(config: &ModulesConfig) -> Self {
        let repositories_module =
            RepositoriesModule::new(&config.repositories).await;
        let services_module = ServicesModule::new(&config.services);

        let user_usecase = UseCase::new(&repositories_module, &services_module);
        let session_usecase =
            UseCase::new(&repositories_module, &services_module);

        Self {
            repositories_module,
            services_module,
            user_usecase,
            session_usecase,
        }
    }
}
