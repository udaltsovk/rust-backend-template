use utoipa::{
    Modify, OpenApi as OpenApiDerive,
    openapi::{
        OpenApi,
        security::{Http, HttpAuthScheme, SecurityScheme},
    },
};

use crate::routes::user::B2C_TAG;

// With api versioning
// #[derive(OpenApiDerive)]
// #[openapi(
//     info(
//         title = "Example Template API"
//     ),
//     servers(
//         (
//             url = "/{api_version}",
//             description = "Default server",
//             variables(
//                 ("api_version" = (
//                     description = "Api version to use",
//                     enum_values("v0"),
//                     default = "v0",
//                 )),
//             ),
//         ),
//     ),
//     tags(
//         (name = B2C_TAG, description = ""),
//     ),
//     modifiers(
//         &SecurityModifier,
//     ),
// )]
// pub struct ApiDoc;

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
