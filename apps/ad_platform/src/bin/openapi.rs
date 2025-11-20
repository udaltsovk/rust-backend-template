use ad_platform::Modules;
use lib::bootstrap::openapi::{OpenAPISaver as _, OpenAPISaverResult};
use presentation::api::rest::routes;

fn main() -> OpenAPISaverResult {
    routes::router::<Modules>()
        .into_openapi()
        .save_as("ad_platform")
}
