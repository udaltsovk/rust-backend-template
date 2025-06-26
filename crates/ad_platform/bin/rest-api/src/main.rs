#[cfg(target_os = "linux")]
use std::ffi::CStr;
use std::net::SocketAddr;

use ad_platform_rest_api::{Modules, config};
use axum_prometheus::PrometheusMetricLayerBuilder;
use infrastructure_instrumentation_opentelemetry::LGTM;
use presentation_api_rest::startup::App;
use tower::ServiceBuilder;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse,
    TraceLayer,
};
use tracing::Level;

#[cfg(target_os = "linux")]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(target_os = "linux")]
#[unsafe(export_name = "malloc_conf")]
pub static MALLOC_CONF: &CStr = c"prof:true,prof_active:true,lg_prof_sample:19";

#[tokio::main]
async fn main() {
    config::init();

    let lgtm = LGTM::init(
        &config::OTEL_ENDPOINT,
        &config::METRICS_ADDRESS,
        "ad_platform",
        "rest-api",
    );

    config::test_values();

    let modules = Modules::new().await;

    let mut app = App::new(modules);

    let middlewares = ServiceBuilder::new()
        .layer(PrometheusMetricLayerBuilder::new().with_prefix("").build())
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(DefaultMakeSpan::new().include_headers(true))
                .on_request(DefaultOnRequest::new().level(Level::INFO))
                .on_response(
                    DefaultOnResponse::new()
                        .level(Level::INFO)
                        .include_headers(true),
                )
                .on_failure(DefaultOnFailure::new().level(Level::WARN)),
        );

    app.router = app.router.layer(middlewares);

    app.run(SocketAddr::from((
        *config::SERVER_ADDRESS,
        *config::SERVER_PORT,
    )))
    .await;
}
