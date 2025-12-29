use std::{ops::Deref as _, sync::Arc};

use opentelemetry_appender_tracing::layer::OpenTelemetryTracingBridge;
#[cfg(any(
    feature = "grpc-tonic",
    feature = "http-proto",
    feature = "http-json",
    test
))]
use opentelemetry_otlp::{LogExporter, WithExportConfig as _};
use opentelemetry_sdk::logs::{
    BatchLogProcessor, LogProcessor, SdkLogger, SdkLoggerProvider,
};
use tap::Pipe as _;

use crate::Otel;

impl Otel {
    pub(super) fn get_logger_provider(&self) -> SdkLoggerProvider {
        self.logger_provider
            .clone()
            .expect("Called `Otel::get_logger_provider` too early")
            .deref()
            .clone()
    }

    #[inline]
    fn log_processor(&self) -> impl LogProcessor + 'static {
        let exporter = {
            #[cfg(feature = "grpc-tonic")]
            {
                LogExporter::builder()
                    .with_tonic()
                    .with_export_config(self.export_config())
                    .build()
                    .expect("Failed to build exporter!")
            }

            #[cfg(all(
                not(feature = "grpc-tonic"),
                any(feature = "http-proto", feature = "http-json", test)
            ))]
            {
                LogExporter::builder()
                    .with_http()
                    .with_export_config(self.export_config())
                    .build()
                    .expect("Failed to build exporter!")
            }

            #[cfg(not(any(
                feature = "grpc-tonic",
                feature = "http-proto",
                feature = "http-json",
                test
            )))]
            #[allow(clippy::cfg_not_test)]
            {
                panic!("No OpenTelemetry protocol selected!");
            }
        };

        BatchLogProcessor::builder(exporter).build()
    }

    #[inline]
    pub(super) fn configure_logger_provider(mut self) -> Self {
        self.logger_provider = SdkLoggerProvider::builder()
            .with_resource(self.resource.clone())
            .with_log_processor(self.log_processor())
            .build()
            .pipe(Arc::new)
            .pipe(Some);

        self
    }

    #[inline]
    pub(super) fn log_layer(
        &self,
    ) -> OpenTelemetryTracingBridge<SdkLoggerProvider, SdkLogger> {
        OpenTelemetryTracingBridge::new(&self.get_logger_provider())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Called `Otel::get_logger_provider` too early")]
    fn get_logger_provider_panic() {
        let otel = Otel::new("test", "test");
        let _provider = otel.get_logger_provider();
    }

    #[tokio::test]
    async fn configure_logger_provider() {
        let otel = Otel::new("test", "test");
        let otel = otel.configure_logger_provider();

        // Should not panic now
        let _provider = otel.get_logger_provider();
    }

    #[tokio::test]
    async fn log_layer() {
        let otel = Otel::new("test", "test");
        let otel = otel.configure_logger_provider();
        let _layer = otel.log_layer();
    }
}
