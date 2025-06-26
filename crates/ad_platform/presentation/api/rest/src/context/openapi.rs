use utoipa::OpenApi;

use crate::routes::clients::CLIENTS_TAG;

#[derive(OpenApi)]
#[openapi(
    info(
        title = "Advertisement Platform API"
    ),
    servers(
        (url = "http://localhost:8080/v1", description = "Local instance (v1)"),
    ),
    tags(
        (name = CLIENTS_TAG, description = ""),
    ),
)]
pub struct ApiDoc;
