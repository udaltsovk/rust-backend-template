#[cfg(target_os = "linux")]
use std::ffi::CStr;

use ad_platform_monolyth::{
    Modules, bootstrappers::BootstraperExt as _, config,
};
use infrastructure_instrumentation_opentelemetry::LGTM;
use presentation_api_rest::startup::RestApi;

#[cfg(target_os = "linux")]
#[global_allocator]
static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

#[cfg(target_os = "linux")]
#[unsafe(export_name = "malloc_conf")]
pub static MALLOC_CONF: &CStr = c"prof:true,prof_active:true,lg_prof_sample:19";

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

            let modules = Modules::new().await;

            RestApi::bootstrap(modules).await;
        },
    )
    .await;
}
