use ad_platform_monolyth::{
    Modules, bootstrappers::BootstraperExt as _, config,
};
use lib::{
    configure_jemalloc, infrastructure::instrumentation::opentelemetry::LGTM,
    presentation::api::rest::startup::RestApi,
};

configure_jemalloc!();

#[tokio::main]
async fn main() {
    config::init();

    LGTM::wrap(
        &config::OTEL_ENDPOINT,
        &config::METRICS_ADDRESS,
        "ad_platform",
        "monolyth",
        async || {
            config::test_values();

            let modules = Modules::init().await;

            RestApi::bootstrap(modules).await;
        },
    )
    .await;
}
