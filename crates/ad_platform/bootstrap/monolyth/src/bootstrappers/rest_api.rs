use std::net::SocketAddr;

use async_trait::async_trait;
use axum_prometheus::PrometheusMetricLayerBuilder;
use presentation_api_rest::startup::RestApi;
use tower::ServiceBuilder;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse,
    TraceLayer,
};
use tracing::Level;

use crate::{Modules, bootstrappers::BootstraperExt, config};

#[async_trait]
impl BootstraperExt for RestApi {
    async fn bootstrap(modules: Modules) {
        let mut api = RestApi::new(modules);

        let middlewares = ServiceBuilder::new()
            .layer(PrometheusMetricLayerBuilder::new().with_prefix("").build())
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(
                        DefaultMakeSpan::new().include_headers(true),
                    )
                    .on_request(DefaultOnRequest::new().level(Level::INFO))
                    .on_response(
                        DefaultOnResponse::new()
                            .level(Level::INFO)
                            .include_headers(true),
                    )
                    .on_failure(DefaultOnFailure::new().level(Level::WARN)),
            );

        api.router = api.router.layer(middlewares);

        api.run(SocketAddr::from((
            *config::SERVER_ADDRESS,
            *config::SERVER_PORT,
        )))
        .await;
    }
}
