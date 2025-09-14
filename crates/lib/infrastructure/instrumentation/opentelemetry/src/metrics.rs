use std::{net::SocketAddr, str::FromStr as _, time::Duration};

use metrics_exporter_prometheus::{Matcher, PrometheusBuilder};
use metrics_tracing_context::TracingContextLayer;
use metrics_util::{MetricKindMask, layers::Layer as _};
use opentelemetry::KeyValue;
use opentelemetry_sdk::Resource;
use opentelemetry_semantic_conventions::attribute;

use crate::LGTM;

pub(super) fn resource(
    otel_service_namespace: &'static str,
    otel_service_name: &'static str,
) -> Resource {
    Resource::builder()
        .with_attributes(vec![
            KeyValue::new(attribute::SERVICE_NAMESPACE, otel_service_namespace),
            KeyValue::new(attribute::SERVICE_NAME, otel_service_name),
        ])
        .build()
}

impl LGTM {
    const HTTP_REQUESTS_DURATION_SECONDS_METRIC_NAME: &str =
        "http_server_request_duration_seconds";
    const METRIC_SCRAPE_INTERVAL: Duration = Duration::from_secs(5);

    pub(super) fn setup_metrics(&self, prometheus_address: &'static str) {
        let (prometheus_recorder, serve_prometheus) = PrometheusBuilder::new()
            .with_http_listener(
                SocketAddr::from_str(prometheus_address)
                    .expect("a valid address"),
            )
            .idle_timeout(
                MetricKindMask::ALL,
                Some(LGTM::METRIC_SCRAPE_INTERVAL),
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
