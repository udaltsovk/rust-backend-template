use application::usecase::{
    session::SessionUseCase as _, user::UserUseCase as _,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use domain::session::entity::SessionEntity;
use lib::{
    presentation::api::rest::{
        context::JsonErrorStruct, extract::Json, model::ParseableJson as _,
        response::ResponseExt as _,
    },
    tap::{Conv as _, Pipe as _},
};

use crate::{
    context::errors::AppError,
    model::user::{CreateJsonUser, JsonUserToken},
    module::ModulesExt,
    routes::user::B2C_TAG,
};

#[utoipa::path(
    post,
    path = "/sign-up",
    tag = B2C_TAG,
    request_body = CreateJsonUser,
    responses(
        (status = OK, body = JsonUserToken),
        (status = BAD_REQUEST, body = JsonErrorStruct),
    ),
)]
pub async fn sign_up<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateJsonUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = source.parse()?;

    let user = modules.user_usecase().register(user).await?;

    modules
        .session_usecase()
        .create(SessionEntity::from(&user))
        .await?
        .conv::<JsonUserToken>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

#[utoipa::path(
    post,
    path = "/log-in",
    tag = B2C_TAG,
    request_body = CreateJsonUser,
    responses(
        (status = OK, body = JsonUserToken),
        (status = BAD_REQUEST, body = JsonErrorStruct),
    ),
)]
pub async fn log_in<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateJsonUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = source.parse()?;

    let user = modules.user_usecase().register(user).await?;

    modules
        .session_usecase()
        .create(SessionEntity::from(&user))
        .await?
        .conv::<JsonUserToken>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
