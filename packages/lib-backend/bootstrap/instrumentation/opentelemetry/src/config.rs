use better_config::{EnvConfig, env};

use crate::LGTM;

#[env(EnvConfig(prefix = "OTEL_"))]
pub struct LgtmConfig {
    #[conf(default = "http://localhost:4317")]
    pub endpoint: String,
    #[conf(default = "template_example")]
    pub service_namespace: String,
    #[conf(default = "monolyth")]
    pub service_name: String,
}

impl From<&LgtmConfig> for LGTM {
    fn from(config: &LgtmConfig) -> Self {
        Self::new(&config.service_namespace, &config.service_name)
            .with_otel_endpoint(&config.endpoint)
    }
}
