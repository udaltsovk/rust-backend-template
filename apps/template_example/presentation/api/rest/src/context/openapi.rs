use utoipa::OpenApi as OpenApiDerive;

use crate::routes::clients::CLIENTS_TAG;

#[derive(OpenApiDerive)]
#[openapi(
    info(
        title = "Example Template API"
    ),
    servers(
        (
            url = "/{api_version}",
            description = "Default server",
            variables(
                ("api_version" = (
                    description = "Api version to use",
                    enum_values("v0"),
                    default = "v0",
                )),
            ),
        ),
    ),
    tags(
        (name = CLIENTS_TAG, description = ""),
    ),
)]
pub struct ApiDoc;
