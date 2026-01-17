use application::usecase::user::UserUseCase as _;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use lib::{
    presentation::api::rest::{
        context::JsonErrorStruct, response::ResponseExt as _,
    },
    tap::{Conv as _, Pipe as _},
};

use crate::{
    ApiError, ModulesExt,
    extractors::{Json, session::UserSession},
    models::user::JsonUser,
    routes::user::B2C_TAG,
};

#[utoipa::path(
    get,
    path = "/profile",
    tag = B2C_TAG,
    security(
        ("user" = []),
    ),
    responses(

        (status = OK, body = JsonUser),
        (status = UNAUTHORIZED, body = JsonErrorStruct),
    ),
)]
pub async fn get_profile<M: ModulesExt>(
    modules: State<M>,
    user_session: UserSession,
) -> Result<impl IntoResponse, ApiError> {
    modules
        .user_usecase()
        .get_by_id(user_session.user_id)
        .await
        .map_err(ApiError::from)?
        .conv::<JsonUser>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
