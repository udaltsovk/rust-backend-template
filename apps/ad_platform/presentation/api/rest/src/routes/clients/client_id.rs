use application::usecase::client::ClientUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::presentation::api::rest::{
    context::JsonErrorStruct,
    extract::{Json, Path},
    response::ResponseExt as _,
};
use tap::Pipe as _;
use uuid::Uuid;

use crate::{
    context::{api_version::ApiVersion, errors::AppError},
    model::client::JsonClient,
    module::ModulesExt,
    routes::clients::CLIENTS_TAG,
};

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
