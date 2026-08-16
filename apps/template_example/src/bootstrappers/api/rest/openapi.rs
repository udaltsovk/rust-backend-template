#![expect(
    clippy::expect_used,
    reason = "startup path: failing fast here is intended"
)]

use utoipa::{
    Modify, OpenApi as OpenApiDerive,
    openapi::{
        OpenApi,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
};

use crate::shared::presentation::api::rest::B2C_TAG;

#[derive(OpenApiDerive)]
#[openapi(
    info(
        title = "Example Template API"
    ),
    servers(
        (
            url = "/",
            description = "Default server",
        ),
    ),
    tags(
        (name = B2C_TAG, description = ""),
    ),
    modifiers(
        &SecurityModifier,
    ),
)]
pub struct ApiDoc;

struct SecurityModifier;
impl Modify for SecurityModifier {
    fn modify(&self, openapi: &mut OpenApi) {
        let components = openapi
            .components
            .as_mut()
            .expect("Failed do get mutable components");

        let scheme = SecurityScheme::Http(
            Http::builder()
                .scheme(HttpAuthScheme::Bearer)
                .bearer_format("JWT")
                .build(),
        );

        components.add_security_scheme("user", scheme);
    }
}
