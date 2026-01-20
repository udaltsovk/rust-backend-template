use std::net::SocketAddr;

use lib::{
    async_trait,
    axum::http::{HeaderValue, Method, header},
    axum_otel_metrics::{HttpMetricsLayerBuilder, PathSkipper},
    presentation::api::rest::startup::RestApi,
    tower::ServiceBuilder,
    tower_http::cors::CorsLayer,
};
use presentation::api::rest::routes;

pub use crate::bootstrappers::rest_api::config::RestApiConfig;
use crate::{Modules, bootstrappers::BootstrapperExt};

mod config;

#[async_trait]
impl BootstrapperExt for RestApi {
    type Config = RestApiConfig;

    async fn bootstrap(config: &Self::Config, modules: Modules) {
        let metric_layer = HttpMetricsLayerBuilder::new()
            .with_skipper(PathSkipper::new(Self::is_openapi_route))
            .build();

        let cors_layer = if config.domain == "localhost" {
            CorsLayer::very_permissive()
        } else {
            CorsLayer::new()
                .allow_origin(config.domain.parse::<HeaderValue>().expect(
                    "`DEPLOY_DOMAIN` value should be parseable `HeaderValue`",
                ))
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
            .layer(ServiceBuilder::new().layer(metric_layer).layer(cors_layer))
            .split_for_parts();

        Self::builder(router, modules)
            .with_openapi(openapi)
            .build()
            .run(SocketAddr::from(config))
            .await;
    }
}
