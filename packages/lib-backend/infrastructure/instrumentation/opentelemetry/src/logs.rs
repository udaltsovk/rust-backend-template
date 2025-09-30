use std::{ops::Deref as _, sync::Arc};

use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
use opentelemetry_otlp::{LogExporter, WithExportConfig as _};
use opentelemetry_sdk::logs::{
    BatchLogProcessor, SdkLogger, SdkLoggerProvider,
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
    pub(super) fn configure_logger_provider(mut self) -> Self {
        let logger_provider = SdkLoggerProvider::builder()
            .with_resource(self.resource.clone())
            .with_log_processor(
                BatchLogProcessor::builder(
                    LogExporter::builder()
                        .with_tonic()
                        .with_export_config(self.export_config())
                        .build()
                        .expect("Failed to build exporter!"),
                )
                .build(),
            )
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
