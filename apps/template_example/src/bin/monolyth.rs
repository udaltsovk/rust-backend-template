use std::{sync::LazyLock, time::Duration};

// use lib::bootstrap::instrumentation::stdout;
use lib::{
    bootstrap::{
        ConfigExt as _, bootstrap, configure_allocator,
        instrumentation::opentelemetry::Otel,
    },
    presentation::api::rest::startup::RestApi,
};
use template_example::{AppConfig, modules::Modules};

configure_allocator!();

static CONFIG: LazyLock<AppConfig> =
    LazyLock::new(AppConfig::load);

#[tokio::main]
async fn main() {
    // // Without opentelemetry
    // stdout::wrap(bootstrap!(
    //     [RestApi(&CONFIG.server)],
    //     Modules::init(&CONFIG.modules)
    // ))
    // .await;

    // With opentelemetry
    Otel::from(&CONFIG.otel)
        .with_timeout(Duration::from_secs(30))
        .wrap(bootstrap!(
            [RestApi(&CONFIG.server)],
            Modules::init(&CONFIG.modules)
        ))
        .await;
}
