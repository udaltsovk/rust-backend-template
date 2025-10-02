use std::{ops::Deref as _, sync::Arc};

use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, WithExportConfig as _};
use opentelemetry_sdk::logs::{
    BatchLogProcessor, LogProcessor, SdkLogger, SdkLoggerProvider,
};

use crate::LGTM;

impl LGTM {
    pub fn get_logger_provider(&self) -> SdkLoggerProvider {
        self.logger_provider
            .clone()
            .expect("Called `LGTM::get_logger_provider` too early")
            .deref()
            .clone()
    }

    #[inline]
    fn log_processor(&self) -> impl LogProcessor + 'static {
        let exporter_builder = LogExporter::builder();

        #[cfg(any(feature = "http-proto", feature = "http-json"))]
        let exporter_builder = exporter_builder.with_http();

        #[cfg(feature = "grpc-tonic")]
        let exporter_builder = exporter_builder.with_tonic();

        BatchLogProcessor::builder(
            exporter_builder
                .with_export_config(self.export_config())
                .build()
                .expect("Failed to build exporter!"),
        )
        .build()
    }

    #[inline]
    pub(super) fn configure_logger_provider(mut self) -> Self {
        let logger_provider = SdkLoggerProvider::builder()
            .with_resource(self.resource.clone())
            .with_log_processor(self.log_processor())
            .build();
        self.logger_provider = Some(Arc::new(logger_provider));
        self
    }

    #[inline]
    pub(super) fn log_layer(
        &self,
    ) -> OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger> {
        OpenTelemetryTracingBridge::new(&self.get_logger_provider())
    }
}
