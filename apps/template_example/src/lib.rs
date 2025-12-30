pub mod bootstrappers;
mod modules;

use fromenv::FromEnv;
use lib::bootstrap::instrumentation::opentelemetry::OtelConfig;
pub use modules::Modules;

use crate::{bootstrappers::rest_api::RestApiConfig, modules::ModulesConfig};

#[derive(FromEnv)]
pub struct AppConfig {
    #[env(nested)]
    pub server: RestApiConfig,
    #[env(nested)]
    pub modules: ModulesConfig,
    #[env(nested)]
    pub otel: OtelConfig,
}
