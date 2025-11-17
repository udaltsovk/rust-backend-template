use application::usecase::client::ClientUseCase as _;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
};
use lib::presentation::api::rest::{
    context::JsonErrorStruct, extract::Json, model::ParseableJson as _,
    response::ResponseExt as _,
};
use tap::Pipe as _;
use utoipa_axum::{router::OpenApiRouter, routes};
use uuid::Uuid;

use crate::{
    context::{api_version::ApiVersion, errors::AppError},
    model::client::{JsonClient, UpsertJsonClient},
    module::ModulesExt,
};

pub const CLIENTS_TAG: &str = "clients";

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new()
        .routes(routes!(bulk_upsert::<M>))
        .routes(routes!(find_by_id::<M>))
}

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

#[utoipa::path(
    get,
    path = "/{client_id}",
    tag = CLIENTS_TAG,
    params(
        ("client_id" = Uuid, Path),
    ),
    responses(
        (status = OK, body = [JsonClient]),
        (status = NOT_FOUND, body = JsonErrorStruct),
    )
)]
pub async fn find_by_id<M>(
    state: State<M>,
    Path((_v, client_id)): Path<(ApiVersion, Uuid)>,
) -> Result<impl IntoResponse, AppError>
where
    M: ModulesExt,
{
    match state
        .client_usecase()
        .find_by_id(client_id.into())
        .await?
        .map(JsonClient::from)
        .map(Json)
    {
        Some(client) => client.into_response().with_status(StatusCode::OK),
        None => JsonErrorStruct::new(
            StatusCode::NOT_FOUND,
            "client_not_found",
            vec![format!("Unable to find client with id `{client_id}`")],
        )
        .into_response(),
    }
    .pipe(Ok)
}
