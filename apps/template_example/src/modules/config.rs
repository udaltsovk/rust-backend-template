use fromenv::FromEnv;

use crate::modules::{
    repositories::RepositoriesConfig, services::ServicesConfig,
};

#[derive(FromEnv)]
pub struct ModulesConfig {
    #[env(nested)]
    pub repositories: RepositoriesConfig,
    #[env(nested)]
    pub services: ServicesConfig,
}
