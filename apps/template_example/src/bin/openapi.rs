use lib::bootstrap::openapi::{OpenAPISaverResult, OpenAPISaverTrait as _};
use presentation::api::rest::routes;
use template_example::Modules;

fn main() -> OpenAPISaverResult {
    routes::router::<Modules>()
        .into_openapi()
        .save_as("template_example")
}
