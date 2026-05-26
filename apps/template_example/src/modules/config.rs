use fromenv::FromEnv;

use super::{
    repositories::RepositoriesConfig,
    services::ServicesConfig,
};

#[derive(FromEnv)]
pub struct ModulesConfig {
    #[env(nested)]
    pub repositories: RepositoriesConfig,
    #[env(nested)]
    pub services: ServicesConfig,
}
