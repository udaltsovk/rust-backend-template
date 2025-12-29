use std::time::Duration;

// use lib::bootstrap::instrumentation::stdout;
use lib::{
    bootstrap::{
        bootstrap, configure_jemalloc, instrumentation::opentelemetry::LGTM,
    },
    presentation::api::rest::startup::RestApi,
};
use template_example::{Config, Modules};

configure_jemalloc!();

#[tokio::main]
async fn main() {
    let config = Config::builder()
        .build()
        .expect("config to be built successfully");

    // Without opentelemetry
    // stdout::wrap(bootstrap!(
    //     template_example,
    //     [RestApi(config.server)],
    //     Modules::init(&config.modules)
    // ))
    // .await;

    // With opentelemetry
    LGTM::from(&config.lgtm)
        .with_otel_timeout(Duration::from_secs(30))
        .wrap(bootstrap!(
            template_example,
            [RestApi(config.server)],
            Modules::init(&config.modules)
        ))
        .await;
}
