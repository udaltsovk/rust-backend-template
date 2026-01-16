use utoipa::OpenApi as OpenApiDerive;

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
    ),
)]
pub struct ApiDoc;
