use std::time::Duration;

use lib::{
    bootstrap::{
        bootstrap, configure_jemalloc, instrumentation::opentelemetry::LGTM,
    },
    presentation::api::rest::startup::RestApi,
};
// use lib::bootstrap::instrumentation::stdout;
use template_example::{Modules, config};

configure_jemalloc!();

#[tokio::main]
async fn main() {
    config::init();

    // Without opentelemetry
    // stdout::init_tracing_subscriber();
    // bootstrap!(template_example, [RestApi], Modules::init()).await;

    // With opentelemetry
    LGTM::new(&config::OTEL_SERVICE_NAMESPACE, &config::OTEL_SERVICE_NAME)
        .with_otel_timeout(Duration::from_secs(30))
        .wrap(bootstrap!(template_example, [RestApi], Modules::init()))
        .await;
}
