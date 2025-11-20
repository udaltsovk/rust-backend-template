use std::{ops::Deref as _, sync::Arc, time::Duration};

use metrics_exporter_otel::OpenTelemetryRecorder;
use metrics_process::Collector;
use opentelemetry::{global, metrics::MeterProvider as _};
use opentelemetry_otlp::{MetricExporter, WithExportConfig as _};
use opentelemetry_sdk::{
    metrics::{
        SdkMeterProvider, periodic_reader_with_async_runtime::PeriodicReader,
        reader::MetricReader,
    },
    runtime,
};
use tap::{Pipe as _, Tap as _};

use crate::LGTM;

impl LGTM {
    const METRIC_SCRAPE_INTERVAL: Duration = Duration::from_secs(1);

    pub(super) fn get_meter_provider(&self) -> SdkMeterProvider {
        self.meter_provider
            .clone()
            .expect("Called `LGTM::get_meter_provider` too early")
            .deref()
            .clone()
    }

    pub(super) fn periodic_reader(&self) -> impl MetricReader + 'static {
        let exporter_builder = MetricExporter::builder();

        #[cfg(any(feature = "http-proto", feature = "http-json"))]
        let exporter_builder = exporter_builder.with_http();

        #[cfg(feature = "grpc-tonic")]
        let exporter_builder = exporter_builder.with_tonic();

        PeriodicReader::builder(
            exporter_builder
                .with_export_config(self.export_config())
                .build()
                .expect("Failed to build exporter!"),
            runtime::Tokio,
        )
        .with_interval(Self::METRIC_SCRAPE_INTERVAL.saturating_mul(10))
        .build()
    }

    #[inline]
    pub(super) fn configure_meter_provider(mut self) -> Self {
        self.meter_provider = SdkMeterProvider::builder()
            .with_resource(self.resource.clone())
            .with_reader(self.periodic_reader())
            .build()
            .tap(|provider| {
                global::set_meter_provider(provider.clone());
            })
            .pipe(Arc::new)
            .pipe(Some);

        self
    }

    pub(super) fn setup_metrics(&self) {
        let meter = self.get_meter_provider().meter(self.otel_service_name);

        ::metrics::set_global_recorder(OpenTelemetryRecorder::new(meter))
            .expect("Failed to set up global metrics recorder");

        let metrics_process_collector = Collector::default();
        metrics_process_collector.describe();

        let interval = Self::METRIC_SCRAPE_INTERVAL;
        tokio::spawn(
            tokio_metrics::RuntimeMetricsReporterBuilder::default()
                .with_interval(interval)
                .describe_and_run(),
        );

        let collector = metrics_process_collector;
        tokio::spawn(async move {
            loop {
                collector.collect();
                tokio::time::sleep(interval).await;
            }
        });
    }
}
