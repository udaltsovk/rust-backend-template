pub mod bootstrappers;
mod modules;

use better_config::{EnvConfig, env};
use lib::bootstrap::instrumentation::opentelemetry::LgtmConfig;
pub use modules::Modules;

use crate::{bootstrappers::rest_api::RestApiConfig, modules::ModulesConfig};

#[env(EnvConfig)]
pub struct Config {
    #[env]
    pub server: RestApiConfig,
    #[env]
    pub modules: ModulesConfig,
    #[env]
    pub lgtm: LgtmConfig,
}
