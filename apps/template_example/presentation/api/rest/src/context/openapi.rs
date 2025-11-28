use utoipa::OpenApi;

use crate::routes::clients::CLIENTS_TAG;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Advertisement Platform API"
    ),
    servers(
        (url = "http://localhost:8080/v0", description = "Local instance (v0)"),
    ),
    tags(
        (name = CLIENTS_TAG, description = ""),
    ),
)]
pub struct ApiDoc;
