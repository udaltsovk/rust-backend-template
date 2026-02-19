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

pub use crate::config::OtelConfig;

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Otel {
    endpoint: Option<String>,
    service_name: String,
    timeout: Option<Duration>,
    resource: Resource,
    logger_provider: Option<Arc<SdkLoggerProvider>>,
    meter_provider: Option<Arc<SdkMeterProvider>>,
    tracer_provider: Option<Arc<SdkTracerProvider>>,
}

impl Otel {
    #[inline]
    const fn protocol() -> Protocol {
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

    fn resource(service_namespace: &str, service_name: &str) -> Resource {
        Resource::builder()
            .with_attribute(KeyValue::new(
                attribute::SERVICE_NAMESPACE,
                service_namespace.to_string(),
            ))
            .with_service_name(service_name.to_string())
            .build()
    }

    #[must_use]
    pub fn new(service_namespace: &str, service_name: &str) -> Self {
        Self {
            service_name: service_name.to_string(),
            resource: Self::resource(service_namespace, service_name),
            endpoint: None,
            timeout: None,
            logger_provider: None,
            meter_provider: None,
            tracer_provider: None,
        }
    }

    #[must_use]
    pub fn with_endpoint(mut self, endpoint: &str) -> Self {
        self.endpoint = Some(endpoint.into());
        self
    }

    #[must_use]
    pub const fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    #[inline]
    fn export_config(&self) -> ExportConfig {
        ExportConfig {
            protocol: Self::protocol(),
            endpoint: self.endpoint.clone(),
            timeout: self.timeout,
        }
    }

    pub async fn wrap<F>(self, future: F)
    where
        F: Future<Output = ()>,
    {
        let otel = self
            .configure_logger_provider()
            .configure_meter_provider()
            .configure_tracer_provider();

        tracing_subscriber::registry()
            .with(stdout::filter_layer())
            .with(stdout::fmt_layer())
            .with(otel.log_layer())
            .with(otel.trace_layer())
            .with(MetricsLayer::new())
            .init();

        otel.setup_metrics();

        future.await;

        tracing::info!("Shutting down OpenTelemetry stuff");

        let result: OTelSdkResult = (|| {
            otel.get_tracer_provider().shutdown()?;
            otel.get_meter_provider().shutdown()?;
            otel.get_logger_provider().shutdown()?;

            Ok(())
        })();

        result.expect("Failed to shut down OpenTelemetry stuff");
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    fn new_and_defaults() {
        let otel = Otel::new("my_ns", "my_svc");
        assert_eq!(otel.service_name, "my_svc");
        assert!(otel.endpoint.is_none());
        assert!(otel.timeout.is_none());
        assert!(otel.logger_provider.is_none());
        assert!(otel.meter_provider.is_none());
        assert!(otel.tracer_provider.is_none());
    }

    #[test]
    fn builder_methods() {
        let timeout = Duration::from_secs(10);
        let endpoint = "http://localhost:4317";

        let otel = Otel::new("my_ns", "my_svc")
            .with_endpoint(endpoint)
            .with_timeout(timeout);

        assert_eq!(otel.endpoint, Some(endpoint.to_string()));
        assert_eq!(otel.timeout, Some(timeout));
    }

    #[test]
    fn export_config() {
        let timeout = Duration::from_secs(10);
        let endpoint = "http://localhost:4317";

        let otel = Otel::new("my_ns", "my_svc")
            .with_endpoint(endpoint)
            .with_timeout(timeout);

        let config = otel.export_config();
        assert_eq!(config.endpoint, Some(endpoint.to_string()));
        assert_eq!(config.timeout, Some(timeout));
    }

    #[test]
    fn resource_creation() {
        let resource = Otel::resource("my_ns", "my_svc");
        assert!(format!("{resource:?}").contains("service.name"));
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn configure_providers() {
        let otel = Otel::new("test_ns", "test_svc");

        let result = tokio::time::timeout(Duration::from_secs(1), async {
            let otel = otel.configure_logger_provider();
            assert!(otel.logger_provider.is_some());
            let _logger_provider = otel.get_logger_provider();
            let _layer = otel.log_layer();

            let otel = otel.configure_meter_provider();
            assert!(otel.meter_provider.is_some());
            let _meter_provider = otel.get_meter_provider();

            let otel = otel.configure_tracer_provider();
            assert!(otel.tracer_provider.is_some());
            let _tracer_provider = otel.get_tracer_provider();

            otel.get_tracer_provider()
                .shutdown()
                .expect("shutdown failed");
            otel.get_meter_provider()
                .shutdown()
                .expect("shutdown failed");
            otel.get_logger_provider()
                .shutdown()
                .expect("shutdown failed");
        })
        .await;

        assert!(result.is_ok(), "Test timed out");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn wrap() {
        let otel = Otel::new("test", "test");
        otel.clone()
            .wrap(async {
                tracing::info!("Inside wrap 1");
            })
            .await;

        // Call wrap again to trigger subscriber initialization error
        otel.wrap(async {
            tracing::info!("Inside wrap 2");
        })
        .await;
    }
}
