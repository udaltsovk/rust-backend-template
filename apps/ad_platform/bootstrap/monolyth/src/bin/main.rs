use std::time::Duration;

use ad_platform_monolyth::{Modules, config};
use lib::{
    bootstrap::{bootstrap, configure_jemalloc},
    infrastructure::instrumentation::opentelemetry::LGTM,
    presentation::api::rest::startup::RestApi,
};

configure_jemalloc!();

#[tokio::main]
async fn main() {
    config::init();

    // Without opentelemetry
    // bootstrap!(ad_platform_monolyth, [RestApi], Modules::init());

    // With opentelemetry
    LGTM::new(&config::OTEL_SERVICE_NAMESPACE, &config::OTEL_SERVICE_NAME)
        .with_otel_timeout(Duration::from_secs(30))
        .wrap(bootstrap!(ad_platform_monolyth, [RestApi], Modules::init()))
        .await
}
