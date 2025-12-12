use std::{ops::Deref as _, sync::Arc, time::Duration};

use metrics_exporter_otel::OpenTelemetryRecorder;
use metrics_process::Collector;
use opentelemetry::{global, metrics::MeterProvider as _};
#[cfg(any(
    feature = "grpc-tonic",
    feature = "http-proto",
    feature = "http-json",
    test
))]
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
        let exporter = {
            #[cfg(feature = "grpc-tonic")]
            {
                MetricExporter::builder()
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
                MetricExporter::builder()
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

        PeriodicReader::builder(exporter, runtime::Tokio)
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

        if let Err(err) =
            ::metrics::set_global_recorder(OpenTelemetryRecorder::new(meter))
        {
            tracing::error!(
                "Failed to set up global metrics recorder: {err:?}"
            );
        }

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

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    #[test]
    #[should_panic(expected = "Called `LGTM::get_meter_provider` too early")]
    fn get_meter_provider_panic() {
        let lgtm = LGTM::new("test", "test");
        let _provider = lgtm.get_meter_provider();
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn configure_meter_provider() {
        let lgtm = LGTM::new("test", "test");

        let result = tokio::time::timeout(Duration::from_secs(1), async {
            let lgtm = lgtm.configure_meter_provider();

            // Should not panic now
            let provider = lgtm.get_meter_provider();

            // Shutdown to clean up
            provider.shutdown().expect("shutdown failed");
        })
        .await;

        assert!(result.is_ok(), "Test timed out");
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn setup_metrics() {
        let lgtm = LGTM::new("test", "test");
        let lgtm = lgtm.configure_meter_provider();
        lgtm.setup_metrics();
    }
}
