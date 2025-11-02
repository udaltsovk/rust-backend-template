use std::net::SocketAddr;

use async_trait::async_trait;
use axum_prometheus::PrometheusMetricLayerBuilder;
use lib::presentation::api::rest::startup::RestApi;
use presentation::api::rest::{context::openapi::ApiDoc, routes};
use tower::ServiceBuilder;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse,
    TraceLayer,
};
use tracing::Level;
use utoipa::OpenApi as _;
use utoipa_axum::router::OpenApiRouter;

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

        let trace_layer = TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().include_headers(true))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(
                DefaultOnResponse::new()
                    .level(Level::INFO)
                    .include_headers(true),
            )
            .on_failure(DefaultOnFailure::new().level(Level::WARN));

        let router = OpenApiRouter::new().nest(
            "/{api_version}",
            routes::router().layer(
                ServiceBuilder::new().layer(metric_layer).layer(trace_layer),
            ),
        );

        RestApi::new(ApiDoc::openapi(), router, modules)
            .run(SocketAddr::from((
                *config::SERVER_ADDRESS,
                *config::SERVER_PORT,
            )))
            .await;
    }
}
