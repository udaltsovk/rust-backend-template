use application::usecase::client::ClientUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::{
    presentation::api::rest::{
        context::JsonErrorStruct, extract::Json, model::ParseableJson as _,
        response::ResponseExt as _,
    },
    tap::Pipe as _,
};

use crate::{
    context::errors::AppError,
    model::client::{JsonClient, UpsertJsonClient},
    module::ModulesExt,
    routes::clients::CLIENTS_TAG,
};

#[utoipa::path(
    post,
    path = "/bulk",
    tag = CLIENTS_TAG,
    request_body = [UpsertJsonClient],
    responses(
        (status = OK, body = [JsonClient]),
        (status = BAD_REQUEST, body = JsonErrorStruct),
    ),
)]
pub async fn bulk_upsert<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<Vec<UpsertJsonClient>>,
) -> Result<impl IntoResponse, AppError> {
    let clients = source.parse()?;
    modules
        .client_usecase()
        .bulk_upsert(&clients)
        .await?
        .into_iter()
        .map(JsonClient::from)
        .collect::<Vec<JsonClient>>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
