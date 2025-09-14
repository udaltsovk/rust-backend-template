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

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct LGTM {
    otel_endpoint: String,
    otel_service_name: Cow<'static, str>,
    resource: Resource,
    logger_provider: Option<Arc<SdkLoggerProvider>>,
    tracer_provider: Option<Arc<SdkTracerProvider>>,
    metrics_process_collector: Arc<Collector>,
}
impl LGTM {
    #[inline]
    fn export_config(&self) -> ExportConfig {
        ExportConfig {
            protocol: Protocol::Grpc,
            endpoint: Some(self.otel_endpoint.clone()),
            timeout: Some(Duration::from_secs(30)),
        }
    }

    pub async fn wrap(
        otel_endpoint: &'static str,
        prometheus_address: &'static str,
        otel_service_namespace: &'static str,
        otel_service_name: &'static str,
        body: impl AsyncFnOnce(),
    ) {
        let lgtm = Self {
            otel_endpoint: otel_endpoint.into(),
            otel_service_name: otel_service_name.into(),
            resource: metrics::resource(
                otel_service_namespace,
                otel_service_name,
            ),
            logger_provider: None,
            tracer_provider: None,
            metrics_process_collector: Arc::new(Collector::default()),
        }
        .configure_logger_provider()
        .configure_tracer_provider();

        tracing_subscriber::registry()
            .with(Self::filter_layer())
            .with(Self::fmt_layer())
            .with(lgtm.log_layer())
            .with(lgtm.trace_layer())
            .with(MetricsLayer::new())
            .init();

        lgtm.setup_metrics(prometheus_address);

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
