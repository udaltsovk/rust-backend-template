use axum::{http::StatusCode, response::IntoResponse};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    context::{
        errors::AppError, response_helper::JsonErrorStruct,
        validate::ValidatedRequest,
    },
    model::time::JsonTime,
    module::ModulesExt,
};

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new().routes(routes!(advance::<M>))
}

#[utoipa::path(
    post,
    path = "/advance",
    request_body = JsonTime,
    responses(
        (status = OK, body = JsonTime),
        (status = BAD_REQUEST, body = JsonErrorStruct),
    )
)]
pub async fn advance<M: ModulesExt>(
    ValidatedRequest(_source): ValidatedRequest<JsonTime>,
) -> Result<impl IntoResponse, AppError> {
    Ok(JsonErrorStruct::new(
        "not_implemented",
        vec!["Method `advance` is not implemented"],
    )
    .as_response(StatusCode::NOT_IMPLEMENTED))
}
