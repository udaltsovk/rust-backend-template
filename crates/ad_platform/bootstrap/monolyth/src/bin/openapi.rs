use std::fs;

use ad_platform_monolyth::Modules;
use presentation_api_rest::routes;

fn main() -> std::io::Result<()> {
    let openapi = routes::router::<Modules>().into_openapi();

    let openapi_json = openapi
        .to_pretty_json()
        .expect("failed to serialize OpenAPI");

    fs::write("./assets/openapi.json", &openapi_json)
}
