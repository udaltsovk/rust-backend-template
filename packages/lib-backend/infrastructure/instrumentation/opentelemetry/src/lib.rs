use std::{borrow::Cow, sync::Arc, time::Duration};

use metrics_process::Collector;
use metrics_tracing_context::MetricsLayer;
use opentelemetry_otlp::{ExportConfig, Protocol};
use opentelemetry_sdk::{
    Resource, error::OTelSdkResult, logs::SdkLoggerProvider,
    trace::SdkTracerProvider,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod layers;
mod logs;
mod metrics;
mod traces;

#[derive(Clone, Debug)]
pub struct LGTM {
    otel_endpoint: Option<String>,
    otel_service_name: Cow<'static, str>,
    otel_timeout: Option<Duration>,
    prometheus_address: &'static str,
    resource: Resource,
    logger_provider: Option<Arc<SdkLoggerProvider>>,
    tracer_provider: Option<Arc<SdkTracerProvider>>,
    metrics_process_collector: Arc<Collector>,
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

    pub fn new(
        prometheus_address: &'static str,
        otel_service_namespace: &'static str,
        otel_service_name: &'static str,
    ) -> Self {
        Self {
            prometheus_address,
            otel_service_name: otel_service_name.into(),
            resource: metrics::resource(
                otel_service_namespace,
                otel_service_name,
            ),
            otel_endpoint: None,
            otel_timeout: None,
            logger_provider: None,
            tracer_provider: None,
            metrics_process_collector: Arc::new(Collector::default()),
        }
    }

    pub fn with_otel_endpoint(mut self, otel_endpoint: &'static str) -> Self {
        self.otel_endpoint = Some(otel_endpoint.into());
        self
    }

    pub fn with_otel_timeout(mut self, otel_timeout: Duration) -> Self {
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

    pub async fn wrap(self, body: impl AsyncFnOnce()) {
        let lgtm = self.configure_logger_provider().configure_tracer_provider();

        tracing_subscriber::registry()
            .with(Self::filter_layer())
            .with(Self::fmt_layer())
            .with(lgtm.log_layer())
            .with(lgtm.trace_layer())
            .with(MetricsLayer::new())
            .init();

        lgtm.setup_metrics();

        body().await;

        tracing::info!("Shutting down LGTM stuff");

        let r: OTelSdkResult = (|| {
            lgtm.get_tracer_provider().shutdown()?;
            lgtm.get_logger_provider().shutdown()?;

            Ok(())
        })();

        r.expect("Failed to shut down LGTM");
    }
}
