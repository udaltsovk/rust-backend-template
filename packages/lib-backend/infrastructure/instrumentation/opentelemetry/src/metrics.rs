use std::{net::SocketAddr, str::FromStr as _, time::Duration};

use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use metrics_tracing_context::TracingContextLayer;
use metrics_util::{MetricKindMask, layers::Layer as _};

use crate::LGTM;

impl LGTM {
    const HTTP_REQUESTS_DURATION_SECONDS_METRIC_NAME: &str =
        "server_http_request_duration_seconds";
    const METRIC_SCRAPE_INTERVAL: Duration = Duration::from_secs(1);

    pub(super) fn setup_metrics(&self) {
        let (prometheus_recorder, serve_prometheus) = PrometheusBuilder::new()
            .add_global_label("service_name", self.otel_service_name.clone())
            .with_http_listener(
                self.prometheus_address.unwrap_or(
                    SocketAddr::from_str("0.0.0.0:8081")
                        .expect("a valid socket address"),
                ),
            )
            .idle_timeout(
                MetricKindMask::ALL,
                Some(LGTM::METRIC_SCRAPE_INTERVAL.saturating_mul(10)),
            )
            .set_buckets_for_metric(
                Matcher::Full(
                    Self::HTTP_REQUESTS_DURATION_SECONDS_METRIC_NAME
                        .to_string(),
                ),
                &[
                    0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0,
                    10.0,
                ],
            )
            .expect("values to be not empty")
            .build()
            .expect("Failed to build Prometheus");

        tokio::spawn(serve_prometheus);

        ::metrics::set_global_recorder(
            TracingContextLayer::all().layer(prometheus_recorder),
        )
        .expect("Failed to set up global metrics recorder");

        self.metrics_process_collector.describe();

        tokio::spawn(
            tokio_metrics::RuntimeMetricsReporterBuilder::default()
                .with_interval(LGTM::METRIC_SCRAPE_INTERVAL)
                .describe_and_run(),
        );

        let collector = self.metrics_process_collector.clone();
        tokio::spawn(async move {
            loop {
                collector.collect();
                tokio::time::sleep(LGTM::METRIC_SCRAPE_INTERVAL).await;
            }
        });
    }
}
