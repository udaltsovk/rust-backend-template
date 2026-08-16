#![expect(
    clippy::expect_used,
    reason = "startup path: failing fast here is intended"
)]

use std::net::SocketAddr;

use entrait::Impl;
use lib::{
    async_trait,
    axum::{
        extract::DefaultBodyLimit,
        http::{HeaderValue, Method, header},
    },
    axum_otel_metrics::{HttpMetricsLayerBuilder, PathSkipper},
    bootstrap::Bootstrapper,
    presentation::api::rest::startup::RestApi,
    tower_http::cors::CorsLayer,
};

pub use self::{config::RestApiConfig, openapi::ApiDoc, routes::router};
use crate::modules::Modules;

mod config;
mod health;
mod openapi;
mod routes;

pub struct PublicApi;

#[async_trait]
impl Bootstrapper for PublicApi {
    type Config = RestApiConfig;
    type Modules = Modules;

    async fn bootstrap(config: &Self::Config, deps: &Impl<Modules>) {
        let metric_layer = HttpMetricsLayerBuilder::new()
            .with_skipper(PathSkipper::new(|path| {
                RestApi::is_openapi_route(path) || health::is_health_route(path)
            }))
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

        let (router, openapi) =
            routes::router().layer(metric_layer).split_for_parts();

        let router = router
            .layer(DefaultBodyLimit::max(config.body_limit_bytes))
            .merge(health::router());

        RestApi::builder(router, deps)
            .with_cors(cors_layer)
            .with_openapi(openapi)
            .build()
            .run(SocketAddr::from(config))
            .await;
    }
}
