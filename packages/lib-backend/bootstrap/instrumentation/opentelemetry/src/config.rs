use better_config::{EnvConfig, env};

use crate::Otel;

#[env(EnvConfig(prefix = "OTEL_"))]
pub struct OtelConfig {
    #[conf(default = "http://localhost:4317")]
    pub endpoint: String,
    #[conf(default = "template_example")]
    pub service_namespace: String,
    #[conf(default = "monolyth")]
    pub service_name: String,
}

impl From<&OtelConfig> for Otel {
    fn from(config: &OtelConfig) -> Self {
        Self::new(&config.service_namespace, &config.service_name)
            .with_endpoint(&config.endpoint)
    }
}
