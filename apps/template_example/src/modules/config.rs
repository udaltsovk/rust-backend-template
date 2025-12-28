use better_config::{EnvConfig, env};

use crate::modules::{
    repositories::RepositoriesConfig, services::ServicesConfig,
};

#[env(EnvConfig)]
pub struct ModulesConfig {
    #[env]
    pub repositories: RepositoriesConfig,
    #[env]
    pub services: ServicesConfig,
}
