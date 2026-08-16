#![expect(
    clippy::expect_used,
    reason = "startup path: failing fast here is intended"
)]

use std::time::Duration;

use metrics_exporter_otel::OpenTelemetryRecorder;
use metrics_process::Collector;
use opentelemetry::{global, metrics::MeterProvider as _};
use opentelemetry_otlp::MetricExporter;
use opentelemetry_sdk::{
    metrics::{
        SdkMeterProvider, periodic_reader_with_async_runtime::PeriodicReader,
        reader::MetricReader,
    },
    runtime,
};
use tap::Tap as _;

use crate::{Otel, Providers};

const METRIC_SCRAPE_INTERVAL: Duration = Duration::from_secs(1);

impl Otel {
    pub(super) fn periodic_reader(&self) -> impl MetricReader + 'static {
        let builder = MetricExporter::builder();

        let exporter = self
            .with_export_config(cfg_select! {
                feature = "grpc-tonic" => builder.with_tonic(),
                _ => builder.with_http(),
            })
            .build()
            .expect("Failed to build exporter!");

        PeriodicReader::builder(exporter, runtime::Tokio)
            .with_interval(METRIC_SCRAPE_INTERVAL.saturating_mul(10))
            .build()
    }

    #[inline]
    pub(super) fn meter_provider(&self) -> SdkMeterProvider {
        SdkMeterProvider::builder()
            .with_resource(self.resource.clone())
            .with_reader(self.periodic_reader())
            .build()
            .tap(|provider| {
                global::set_meter_provider(provider.clone());
            })
    }
}

impl Providers {
    pub(super) fn setup_metrics(&self) {
        let meter = self.meter.meter(self.service_name.clone().leak());

        if let Err(err) =
            ::metrics::set_global_recorder(OpenTelemetryRecorder::new(meter))
        {
            tracing::error!(
                "Failed to set up global metrics recorder: {err:?}"
            );
        }

        let metrics_process_collector = Collector::default();
        metrics_process_collector.describe();

        let interval = METRIC_SCRAPE_INTERVAL;
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
