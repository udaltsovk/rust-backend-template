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

mod layers;
mod logs;
mod metrics;
mod traces;

#[derive(Clone, Debug)]
pub struct LGTM {
    otel_endpoint: Option<String>,
    otel_service_name: &'static str,
    otel_timeout: Option<Duration>,
    resource: Resource,
    logger_provider: Option<Arc<SdkLoggerProvider>>,
    meter_provider: Option<Arc<SdkMeterProvider>>,
    tracer_provider: Option<Arc<SdkTracerProvider>>,
}

impl LGTM {
    const OTEL_PROTO: Protocol = if cfg!(any(
        all(
            feature = "http-proto",
            feature = "http-json",
            feature = "grpc-tonic"
        ),
        all(feature = "http-proto", feature = "http-json"),
        all(feature = "http-json", feature = "grpc-tonic"),
        all(feature = "http-proto", feature = "grpc-tonic")
    )) {
        panic!("Multiple OpenTelemetry protocols selected!")
    } else if cfg!(feature = "http-proto") {
        Protocol::HttpBinary
    } else if cfg!(feature = "http-json") {
        Protocol::HttpJson
    } else if cfg!(feature = "grpc-tonic") {
        Protocol::Grpc
    } else {
        panic!("No OpenTelemetry protocol selected!") // that shouldn't happen
    };

    fn resource(
        otel_service_namespace: &'static str,
        otel_service_name: &'static str,
    ) -> Resource {
        Resource::builder()
            .with_attribute(KeyValue::new(
                attribute::SERVICE_NAMESPACE,
                otel_service_namespace,
            ))
            .with_service_name(otel_service_name)
            .build()
    }

    #[must_use]
    pub fn new(
        otel_service_namespace: &'static str,
        otel_service_name: &'static str,
    ) -> Self {
        Self {
            otel_service_name,
            resource: Self::resource(otel_service_namespace, otel_service_name),
            otel_endpoint: None,
            otel_timeout: None,
            logger_provider: None,
            meter_provider: None,
            tracer_provider: None,
        }
    }

    #[must_use]
    pub fn with_otel_endpoint(mut self, otel_endpoint: &'static str) -> Self {
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
            protocol: Self::OTEL_PROTO,
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

        tracing_subscriber::registry()
            .with(Self::filter_layer())
            .with(Self::fmt_layer())
            .with(lgtm.log_layer())
            .with(lgtm.trace_layer())
            .with(MetricsLayer::new())
            .init();

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
