#![expect(
    clippy::expect_used,
    reason = "startup path: failing fast here is intended"
)]

use opentelemetry::{global, trace::TracerProvider as _};
use opentelemetry_otlp::SpanExporter;
use opentelemetry_sdk::trace::{
    BatchSpanProcessor, SdkTracerProvider, SpanProcessor, Tracer,
};
use tap::Tap as _;
use tracing::Subscriber;
use tracing_opentelemetry::OpenTelemetryLayer;
use tracing_subscriber::registry::LookupSpan;

use crate::{Otel, Providers};

impl Otel {
    #[inline]
    fn span_processor(&self) -> impl SpanProcessor + 'static {
        let builder = SpanExporter::builder();

        let exporter = self
            .with_export_config(cfg_select! {
                feature = "grpc-tonic" => builder.with_tonic(),
                _ => builder.with_http(),
            })
            .build()
            .expect("Failed to build exporter!");

        BatchSpanProcessor::builder(exporter).build()
    }

    #[inline]
    pub(super) fn tracer_provider(&self) -> SdkTracerProvider {
        SdkTracerProvider::builder()
            .with_resource(self.resource.clone())
            .with_span_processor(self.span_processor())
            .build()
            .tap(|provider| {
                global::set_tracer_provider(provider.clone());
            })
    }
}

impl Providers {
    #[inline]
    pub(super) fn trace_layer<S: Subscriber + for<'span> LookupSpan<'span>>(
        &self,
    ) -> OpenTelemetryLayer<S, Tracer> {
        OpenTelemetryLayer::new(self.tracer.tracer(self.service_name.clone()))
    }
}
