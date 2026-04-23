#![feature(try_blocks)]

use std::{sync::Arc, time::Duration};

use metrics_tracing_context::MetricsLayer;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{ExportConfig, Protocol};
use opentelemetry_sdk::{
    Resource, logs::SdkLoggerProvider,
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

#[derive(Clone, Debug)]
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
        } else {
            panic!("No OpenTelemetry protocol selected!") // that shouldn't happen
        }
    }

    fn resource(
        service_namespace: &str,
        service_name: &str,
    ) -> Resource {
        Resource::builder()
            .with_attribute(KeyValue::new(
                attribute::SERVICE_NAMESPACE,
                service_namespace.to_string(),
            ))
            .with_service_name(service_name.to_string())
            .build()
    }

    #[must_use]
    pub fn new(
        service_namespace: &str,
        service_name: &str,
    ) -> Self {
        Self {
            service_name: service_name.to_string(),
            resource: Self::resource(
                service_namespace,
                service_name,
            ),
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
    pub const fn with_timeout(
        mut self,
        timeout: Duration,
    ) -> Self {
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

        try {
            otel.get_tracer_provider().shutdown()?;
            otel.get_meter_provider().shutdown()?;
            otel.get_logger_provider().shutdown()?;
        }
        .expect("Failed to shut down OpenTelemetry stuff");
    }
}
