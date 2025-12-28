use std::{sync::Arc, time::Duration};

use metrics_tracing_context::MetricsLayer;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{ExportConfig, Protocol};
use opentelemetry_sdk::{
    Resource, error::OTelSdkResult, logs::SdkLoggerProvider,
    metrics::SdkMeterProvider, trace::SdkTracerProvider,
};
use opentelemetry_semantic_conventions::attribute;
use tracing_subscriber::{
    layer::SubscriberExt as _, util::SubscriberInitExt as _,
};

mod config;
mod logs;
mod metrics;
mod traces;

pub use crate::config::LgtmConfig;

#[derive(Clone, Debug)]
pub struct LGTM {
    otel_endpoint: Option<String>,
    otel_service_name: String,
    otel_timeout: Option<Duration>,
    resource: Resource,
    logger_provider: Option<Arc<SdkLoggerProvider>>,
    meter_provider: Option<Arc<SdkMeterProvider>>,
    tracer_provider: Option<Arc<SdkTracerProvider>>,
}

impl LGTM {
    fn otel_protocol() -> Protocol {
        if cfg!(feature = "grpc-tonic") {
            Protocol::Grpc
        } else if cfg!(feature = "http-proto") {
            Protocol::HttpBinary
        } else if cfg!(feature = "http-json") {
            Protocol::HttpJson
        } else if cfg!(test) {
            Protocol::HttpBinary
        } else {
            panic!("No OpenTelemetry protocol selected!") // that shouldn't happen
        }
    }

    fn resource(
        otel_service_namespace: &str,
        otel_service_name: &str,
    ) -> Resource {
        Resource::builder()
            .with_attribute(KeyValue::new(
                attribute::SERVICE_NAMESPACE,
                otel_service_namespace.to_string(),
            ))
            .with_service_name(otel_service_name.to_string())
            .build()
    }

    #[must_use]
    pub fn new(otel_service_namespace: &str, otel_service_name: &str) -> Self {
        Self {
            otel_service_name: otel_service_name.to_string(),
            resource: Self::resource(otel_service_namespace, otel_service_name),
            otel_endpoint: None,
            otel_timeout: None,
            logger_provider: None,
            meter_provider: None,
            tracer_provider: None,
        }
    }

    #[must_use]
    pub fn with_otel_endpoint(mut self, otel_endpoint: &str) -> Self {
        self.otel_endpoint = Some(otel_endpoint.into());
        self
    }

    #[must_use]
    pub const fn with_otel_timeout(mut self, otel_timeout: Duration) -> Self {
        self.otel_timeout = Some(otel_timeout);
        self
    }

    #[inline]
    fn export_config(&self) -> ExportConfig {
        ExportConfig {
            protocol: Self::otel_protocol(),
            endpoint: self.otel_endpoint.clone(),
            timeout: self.otel_timeout,
        }
    }

    pub async fn wrap<F>(self, future: F)
    where
        F: Future<Output = ()>,
    {
        let lgtm = self
            .configure_logger_provider()
            .configure_meter_provider()
            .configure_tracer_provider();

        if let Err(err) = tracing_subscriber::registry()
            .with(stdout::filter_layer())
            .with(stdout::fmt_layer())
            .with(lgtm.log_layer())
            .with(lgtm.trace_layer())
            .with(MetricsLayer::new())
            .try_init()
        {
            tracing::error!("Failed to initialize tracing subscriber: {err:?}");
        }

        lgtm.setup_metrics();

        future.await;

        tracing::info!("Shutting down LGTM stuff");

        let result: OTelSdkResult = (|| {
            lgtm.get_tracer_provider().shutdown()?;
            lgtm.get_meter_provider().shutdown()?;
            lgtm.get_logger_provider().shutdown()?;

            Ok(())
        })();

        result.expect("Failed to shut down LGTM");
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn new_and_defaults() {
        let lgtm = LGTM::new("my_ns", "my_svc");
        assert_eq!(lgtm.otel_service_name, "my_svc");
        assert!(lgtm.otel_endpoint.is_none());
        assert!(lgtm.otel_timeout.is_none());
        assert!(lgtm.logger_provider.is_none());
        assert!(lgtm.meter_provider.is_none());
        assert!(lgtm.tracer_provider.is_none());
    }

    #[test]
    fn builder_methods() {
        let timeout = Duration::from_secs(10);
        let endpoint = "http://localhost:4317";

        let lgtm = LGTM::new("my_ns", "my_svc")
            .with_otel_endpoint(endpoint)
            .with_otel_timeout(timeout);

        assert_eq!(lgtm.otel_endpoint, Some(endpoint.to_string()));
        assert_eq!(lgtm.otel_timeout, Some(timeout));
    }

    #[test]
    fn export_config() {
        let timeout = Duration::from_secs(10);
        let endpoint = "http://localhost:4317";

        let lgtm = LGTM::new("my_ns", "my_svc")
            .with_otel_endpoint(endpoint)
            .with_otel_timeout(timeout);

        let config = lgtm.export_config();
        assert_eq!(config.endpoint, Some(endpoint.to_string()));
        assert_eq!(config.timeout, Some(timeout));
    }

    #[test]
    fn resource_creation() {
        let resource = LGTM::resource("my_ns", "my_svc");
        assert!(format!("{resource:?}").contains("service.name"));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn configure_providers() {
        let lgtm = LGTM::new("test_ns", "test_svc");

        let result = tokio::time::timeout(Duration::from_secs(1), async {
            let lgtm = lgtm.configure_logger_provider();
            assert!(lgtm.logger_provider.is_some());
            let _logger_provider = lgtm.get_logger_provider();
            let _layer = lgtm.log_layer();

            let lgtm = lgtm.configure_meter_provider();
            assert!(lgtm.meter_provider.is_some());
            let _meter_provider = lgtm.get_meter_provider();

            let lgtm = lgtm.configure_tracer_provider();
            assert!(lgtm.tracer_provider.is_some());
            let _tracer_provider = lgtm.get_tracer_provider();

            lgtm.get_tracer_provider()
                .shutdown()
                .expect("shutdown failed");
            lgtm.get_meter_provider()
                .shutdown()
                .expect("shutdown failed");
            lgtm.get_logger_provider()
                .shutdown()
                .expect("shutdown failed");
        })
        .await;

        assert!(result.is_ok(), "Test timed out");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn wrap() {
        let lgtm = LGTM::new("test", "test");
        lgtm.clone()
            .wrap(async {
                tracing::info!("Inside wrap 1");
            })
            .await;

        // Call wrap again to trigger subscriber initialization error
        lgtm.wrap(async {
            tracing::info!("Inside wrap 2");
        })
        .await;
    }
}
