use std::time::Duration;

// use lib::bootstrap::instrumentation::stdout;
use lib::{
    bootstrap::{
        ConfigExt as _, bootstrap, configure_jemalloc,
        instrumentation::opentelemetry::Otel,
    },
    presentation::api::rest::startup::RestApi,
};
use template_example::{AppConfig, Modules};

configure_jemalloc!();

#[tokio::main]
async fn main() {
    let config = AppConfig::load();

    // Without opentelemetry
    // stdout::wrap(bootstrap!(
    //     template_example,
    //     [RestApi(&config.server)],
    //     Modules::init(&config.modules)
    // ))
    // .await;

    // With opentelemetry
    Otel::from(&config.otel)
        .with_timeout(Duration::from_secs(30))
        .wrap(bootstrap!(
            template_example,
            [RestApi(&config.server)],
            Modules::init(&config.modules)
        ))
        .await;
}
