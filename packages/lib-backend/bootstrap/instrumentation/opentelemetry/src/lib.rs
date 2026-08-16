#![feature(try_blocks)]
#![expect(
    clippy::expect_used,
    reason = "startup path: failing fast here is intended"
)]

use std::time::Duration;

use metrics_tracing_context::MetricsLayer;
use opentelemetry::KeyValue;
use opentelemetry_otlp::{Protocol, WithExportConfig};
use opentelemetry_sdk::{
    Resource, logs::SdkLoggerProvider, metrics::SdkMeterProvider,
    trace::SdkTracerProvider,
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
}

pub(crate) struct Providers {
    pub(crate) service_name: String,
    pub(crate) logger: SdkLoggerProvider,
    pub(crate) meter: SdkMeterProvider,
    pub(crate) tracer: SdkTracerProvider,
}

impl Otel {
    #[inline]
    const fn protocol() -> Protocol {
        cfg_select! {
            feature = "grpc-tonic" => Protocol::Grpc,
            feature = "http-proto" => Protocol::HttpBinary,
            feature = "http-json" => Protocol::HttpJson,
            _ => compile_error!("no OpenTelemetry protocol feature selected"),
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
    fn with_export_config<B: WithExportConfig>(&self, builder: B) -> B {
        let builder = builder.with_protocol(Self::protocol());

        let builder = match self.endpoint.as_deref() {
            Some(endpoint) => builder.with_endpoint(endpoint),
            None => builder,
        };

        match self.timeout {
            Some(timeout) => builder.with_timeout(timeout),
            None => builder,
        }
    }

    pub async fn wrap<F>(self, future: F)
    where
        F: Future<Output = ()>,
    {
        let providers = Providers {
            service_name: self.service_name.clone(),
            logger: self.logger_provider(),
            meter: self.meter_provider(),
            tracer: self.tracer_provider(),
        };

        tracing_subscriber::registry()
            .with(stdout::filter_layer())
            .with(stdout::fmt_layer())
            .with(providers.log_layer())
            .with(providers.trace_layer())
            .with(MetricsLayer::new())
            .init();

        providers.setup_metrics();

        future.await;

        tracing::info!("Shutting down OpenTelemetry stuff");

        try {
            providers.tracer.shutdown()?;
            providers.meter.shutdown()?;
            providers.logger.shutdown()?;
        }
        .expect("Failed to shut down OpenTelemetry stuff");
    }
}
