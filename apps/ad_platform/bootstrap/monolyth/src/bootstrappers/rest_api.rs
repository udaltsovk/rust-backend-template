use std::net::SocketAddr;

use async_trait::async_trait;
use axum::Router;
use axum_otel_metrics::{HttpMetricsLayerBuilder, PathSkipper};
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use lib::presentation::api::rest::startup::RestApi;
use presentation::api::rest::routes;
use tower::ServiceBuilder;

use crate::{Modules, bootstrappers::BootstrapperExt, config};

#[async_trait]
impl BootstrapperExt for RestApi {
    async fn bootstrap(modules: Modules) {
        let metric_layer = HttpMetricsLayerBuilder::new()
            .with_skipper(PathSkipper::new(Self::is_openapi_route))
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
            *config::SERVER_HOST,
            *config::SERVER_PORT,
        )))
        .await;
    }
}
