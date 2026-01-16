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
    AppError, ModulesExt,
    errors::BadRequestResponse,
    models::{
        session::{CreateJsonSession, JsonUserSession},
        user::CreateJsonUser,
    },
    routes::user::B2C_TAG,
};

/// Регистрация нового пользователя
///
/// Регистрирует нового пользователя и возвращает токен доступа.
#[utoipa::path(
    post,
    path = "/sign-up",
    tag = B2C_TAG,
    request_body = CreateJsonUser,
    responses(
        (
            status = OK,
            body = JsonUserSession,
            description = "Пользователь успешно зарегистрирован."
        ),
        (
            status = CONFLICT,
            body = JsonErrorStruct,
            description = "Такой email уже зарегистрирован в системе"
        ),
        BadRequestResponse
    ),
)]
pub async fn sign_up<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateJsonUser>,
) -> Result<impl IntoResponse, AppError> {
    let user = source.parse()?;

    let user = modules.user_usecase().create(user).await?;

    modules
        .session_usecase()
        .create(SessionEntity::from(&user))
        .await?
        .conv::<JsonUserSession>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

#[utoipa::path(
    post,
    path = "/sign-in",
    tag = B2C_TAG,
    request_body = CreateJsonSession,
    responses(
        (
            status = OK,
            body = JsonUserSession
        ),
        (
            status = UNAUTHORIZED,
            body = JsonErrorStruct
        ),
        BadRequestResponse
    ),
)]
pub async fn log_in<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateJsonSession>,
) -> Result<impl IntoResponse, AppError> {
    let credentials = source.parse()?;

    let user = modules.user_usecase().authorize(credentials).await?;

    modules
        .session_usecase()
        .create(SessionEntity::from(&user))
        .await?
        .conv::<JsonUserSession>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
