use fromenv::FromEnv;

use crate::Otel;

#[derive(FromEnv)]
#[env(prefix = "OTEL_")]
pub struct OtelConfig {
    pub endpoint: String,
    pub service_namespace: String,
    pub service_name: String,
}

impl From<&OtelConfig> for Otel {
    fn from(config: &OtelConfig) -> Self {
        Self::new(&config.service_namespace, &config.service_name)
            .with_endpoint(&config.endpoint)
    }
}
