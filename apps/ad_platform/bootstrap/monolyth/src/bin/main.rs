use std::time::Duration;

use ad_platform_monolyth::{
    Modules, bootstrappers::BootstraperExt as _, config,
};
use lib::{
    configure_jemalloc, infrastructure::instrumentation::opentelemetry::LGTM,
    presentation::api::rest::startup::RestApi,
};

configure_jemalloc!();

async fn start() {
    config::test_values();

    let modules = Modules::init().await;

    tokio::join!(RestApi::bootstrap(modules));
}

#[tokio::main]
async fn main() {
    config::init();

    // Without opentelemetry
    // start().await

    // With opentelemetry
    LGTM::new(&config::OTEL_SERVICE_NAMESPACE, &config::OTEL_SERVICE_NAME)
        .with_prometheus_address(&config::PROMETHEUS_ADDRESS)
        .with_otel_timeout(Duration::from_secs(30))
        .wrap(start)
        .await
}
