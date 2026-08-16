use std::{sync::LazyLock, time::Duration};

// use lib::bootstrap::instrumentation::stdout;
use lib::bootstrap::{
    ConfigExt as _, bootstrap, configure_allocator,
    instrumentation::opentelemetry::Otel,
};
use template_example::{
    AppConfig, bootstrappers::api::rest::PublicApi, modules::Modules,
};

configure_allocator!();

static CONFIG: LazyLock<AppConfig> = LazyLock::new(AppConfig::load);

#[tokio::main]
async fn main() {
    // // Without opentelemetry
    // stdout::wrap(bootstrap!(
    //     [PublicApi(&CONFIG.server)],
    //     Modules::init(&CONFIG.modules)
    // ))
    // .await;

    // With opentelemetry
    Otel::from(&CONFIG.otel)
        .with_timeout(Duration::from_secs(30))
        .wrap(bootstrap!(
            [PublicApi(&CONFIG.server)],
            Modules::init(&CONFIG.modules)
        ))
        .await;
}
