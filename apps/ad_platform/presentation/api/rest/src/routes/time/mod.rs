use axum::{Json, http::StatusCode, response::IntoResponse};
use lib::presentation::api::rest::context::JsonErrorStruct;
use tap::Pipe as _;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    context::errors::AppError, model::time::JsonTime, module::ModulesExt,
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
    Json(_source): Json<JsonTime>,
) -> Result<impl IntoResponse, AppError> {
    JsonErrorStruct::new(
        "not_implemented",
        vec!["Method `advance` is not implemented"],
    )
    .as_response(StatusCode::NOT_IMPLEMENTED)
    .pipe(Ok)
}
