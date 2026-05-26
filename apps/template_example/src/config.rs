use fromenv::FromEnv;
use lib::bootstrap::instrumentation::opentelemetry::OtelConfig;

use crate::{
    bootstrappers::api::rest::RestApiConfig,
    modules::ModulesConfig,
};

#[derive(FromEnv)]
pub struct AppConfig {
    #[env(nested)]
    pub server: RestApiConfig,
    #[env(nested)]
    pub modules: ModulesConfig,
    #[env(nested)]
    pub otel: OtelConfig,
}
