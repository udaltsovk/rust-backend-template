use std::{ops::Deref as _, sync::Arc};

use opentelemetry::{global, trace::TracerProvider as _};
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
        let exporter_builder = SpanExporter::builder();

        #[cfg(any(feature = "http-proto", feature = "http-json"))]
        let exporter_builder = exporter_builder.with_http();

        #[cfg(feature = "grpc-tonic")]
        let exporter_builder = exporter_builder.with_tonic();

        BatchSpanProcessor::builder(
            exporter_builder
                .with_export_config(self.export_config())
                .build()
                .expect("Failed to build exporter!"),
        )
        .build()
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
            self.tracer_provider
                .clone()
                .expect("Called `LGTM::trace_layer` too early")
                .tracer(self.otel_service_name.clone()),
        )
    }
}
