#![expect(
    clippy::expect_used,
    reason = "startup path: failing fast here is intended"
)]

use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::LogExporter;
use opentelemetry_sdk::logs::{
    BatchLogProcessor, LogProcessor, SdkLogger, SdkLoggerProvider,
};

use crate::{Otel, Providers};

impl Otel {
    #[inline]
    fn log_processor(&self) -> impl LogProcessor + 'static {
        let builder = LogExporter::builder();

        let exporter = self
            .with_export_config(cfg_select! {
                feature = "grpc-tonic" => builder.with_tonic(),
                _ => builder.with_http(),
            })
            .build()
            .expect("Failed to build exporter!");

        BatchLogProcessor::builder(exporter).build()
    }

    #[inline]
    pub(super) fn logger_provider(&self) -> SdkLoggerProvider {
        SdkLoggerProvider::builder()
            .with_resource(self.resource.clone())
            .with_log_processor(self.log_processor())
            .build()
    }
}

impl Providers {
    #[inline]
    pub(super) fn log_layer(
        &self,
    ) -> OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger> {
        OpenTelemetryTracingBridge::new(&self.logger)
    }
}
