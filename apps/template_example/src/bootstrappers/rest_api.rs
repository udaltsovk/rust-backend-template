use std::net::SocketAddr;

use async_trait::async_trait;
use axum::{
    Router,
    http::{HeaderValue, Method, header},
};
use axum_otel_metrics::{HttpMetricsLayerBuilder, PathSkipper};
use axum_tracing_opentelemetry::middleware::OtelAxumLayer;
use lib::presentation::api::rest::startup::RestApi;
use presentation::api::rest::routes;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

use crate::{Modules, bootstrappers::BootstrapperExt, config};

#[async_trait]
impl BootstrapperExt for RestApi {
    async fn bootstrap(modules: Modules) {
        let metric_layer = HttpMetricsLayerBuilder::new()
            .with_skipper(PathSkipper::new(Self::is_openapi_route))
            .build();

        let cors_layer = if *config::DEPLOY_DOMAIN == "localhost" {
            CorsLayer::very_permissive()
        } else {
            CorsLayer::new()
                .allow_origin(
                    config::DEPLOY_DOMAIN.parse::<HeaderValue>().expect(
                        "`DEPLOY_DOMAIN` value to be parseable `HeaderValue`",
                    ),
                )
                .allow_methods([
                    Method::GET,
                    Method::POST,
                    Method::PUT,
                    Method::PATCH,
                    Method::DELETE,
                ])
                .allow_headers([
                    header::AUTHORIZATION,
                    header::ACCEPT,
                    header::ACCEPT_ENCODING,
                ])
                .allow_credentials(true)
        };

        let (router, openapi) = routes::router()
            .layer(
                ServiceBuilder::new()
                    .layer(metric_layer)
                    .layer(OtelAxumLayer::default())
                    .layer(cors_layer),
            )
            .split_for_parts();

        Self::builder(Router::new().nest("/{api_version}", router), modules)
            .with_openapi(openapi)
            .build()
            .run(SocketAddr::from((
                *config::SERVER_HOST,
                *config::SERVER_PORT,
            )))
            .await;
    }
}
