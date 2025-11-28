use lib::bootstrap::openapi::{OpenAPISaver as _, OpenAPISaverResult};
use presentation::api::rest::routes;
use template_example::Modules;

fn main() -> OpenAPISaverResult {
    routes::router::<Modules>()
        .into_openapi()
        .save_as("template_example")
}
