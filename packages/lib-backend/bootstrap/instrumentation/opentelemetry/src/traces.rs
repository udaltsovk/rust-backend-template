use std::{ops::Deref as _, sync::Arc};

use opentelemetry::{global, trace::TracerProvider as _};
#[cfg(any(
    feature = "grpc-tonic",
    feature = "http-proto",
    feature = "http-json",
    test
))]
use opentelemetry_otlp::{SpanExporter, WithExportConfig as _};
use opentelemetry_sdk::trace::{
    BatchSpanProcessor, SdkTracerProvider, SpanProcessor, Tracer,
};
use tap::{Pipe as _, Tap as _};
use tracing::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::registry::LookupSpan;

use crate::LGTM;

impl LGTM {
    pub(super) fn get_tracer_provider(&self) -> SdkTracerProvider {
        self.tracer_provider
            .clone()
            .expect("Called `LGTM::get_tracer_provider` too early")
            .deref()
            .clone()
    }

    #[inline]
    fn span_processor(&self) -> impl SpanProcessor + 'static {
        let exporter = {
            #[cfg(feature = "grpc-tonic")]
            {
                SpanExporter::builder()
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
                SpanExporter::builder()
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

        BatchSpanProcessor::builder(exporter).build()
    }

    #[inline]
    pub(super) fn configure_tracer_provider(mut self) -> Self {
        self.tracer_provider = SdkTracerProvider::builder()
            .with_resource(self.resource.clone())
            .with_span_processor(self.span_processor())
            .build()
            .tap(|provider| {
                global::set_tracer_provider(provider.clone());
            })
            .pipe(Arc::new)
            .pipe(Some);

        self
    }

    #[inline]
    pub(super) fn trace_layer<S: Subscriber + for<'span> LookupSpan<'span>>(
        &self,
    ) -> OpenTelemetryLayer<S, Tracer> {
        OpenTelemetryLayer::new(
            self.get_tracer_provider().tracer(self.otel_service_name),
        )
    }
}

#[cfg(test)]
mod tests {
    use tracing_subscriber::Registry;

    use super::*;

    #[test]
    #[should_panic(expected = "Called `LGTM::get_tracer_provider` too early")]
    fn get_tracer_provider_panic() {
        let lgtm = LGTM::new("test", "test");
        let _provider = lgtm.get_tracer_provider();
    }

    #[tokio::test]
    async fn configure_tracer_provider() {
        let lgtm = LGTM::new("test", "test");
        let lgtm = lgtm.configure_tracer_provider();

        // Should not panic now
        let _provider = lgtm.get_tracer_provider();
    }

    #[tokio::test]
    async fn trace_layer() {
        let lgtm = LGTM::new("test", "test");
        let lgtm = lgtm.configure_tracer_provider();
        let _layer = lgtm.trace_layer::<Registry>();
    }
}
