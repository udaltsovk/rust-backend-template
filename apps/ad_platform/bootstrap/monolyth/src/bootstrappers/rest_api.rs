use std::net::SocketAddr;

use async_trait::async_trait;
use axum::Router;
use axum_prometheus::PrometheusMetricLayerBuilder;
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use lib::presentation::api::rest::startup::RestApi;
use presentation::api::rest::routes;
use tower::ServiceBuilder;

use crate::{Modules, bootstrappers::BootstraperExt, config};

#[async_trait]
impl BootstraperExt for RestApi {
    async fn bootstrap(modules: Modules) {
        let metric_layer = PrometheusMetricLayerBuilder::new()
            .with_prefix("server")
            .with_ignore_patterns(&[
                "/openapi",
                "/openapi.json",
                "/favicon.ico",
            ])
            .build();

        let (router, openapi) = routes::router()
            .layer(
                ServiceBuilder::new()
                    .layer(metric_layer)
                    .layer(OtelAxumLayer::default()),
            )
            .split_for_parts();

        RestApi::new(
            openapi,
            Router::new().nest("/{api_version}", router),
            modules,
        )
        .run(SocketAddr::from((
            *config::SERVER_ADDRESS,
            *config::SERVER_PORT,
        )))
        .await;
    }
}
