use application::usecase::{
    session::SessionUseCase as _, user::UserUseCase as _,
};
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use domain::session::entity::SessionEntity;
use lib::{
    presentation::api::rest::{
        errors::JsonError, response::ResponseExt as _,
        validation::parseable::Parseable as _,
    },
    tap::{Conv as _, Pipe as _},
};

use crate::{
    ApiError, ModulesExt,
    dto::{
        session::{CreateSessionDto, SessionDto},
        user::CreateUserDto,
    },
    errors::{BadRequestResponse, ValidationFailedResponse},
    extractors::Json,
    routes::user::B2C_TAG,
};

/// Регистрация нового пользователя
///
/// Регистрирует нового пользователя и возвращает токен доступа.
#[utoipa::path(
    post,
    path = "/sign-up",
    tag = B2C_TAG,
    request_body = CreateUserDto,
    responses(
        (
            status = OK,
            body = SessionDto,
            description = "Пользователь успешно зарегистрирован."
        ),
        (
            status = CONFLICT,
            body = JsonError,
            description = "Такой email уже зарегистрирован в системе"
        ),
        ValidationFailedResponse,
        BadRequestResponse
    ),
)]
pub async fn sign_up<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateUserDto>,
) -> Result<impl IntoResponse, ApiError> {
    let user = source.parse()?;

    let user = modules.user_usecase().create(user).await?;

    modules
        .session_usecase()
        .create(SessionEntity::from(&user))
        .await?
        .conv::<SessionDto>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}

#[utoipa::path(
    post,
    path = "/sign-in",
    tag = B2C_TAG,
    request_body = CreateSessionDto,
    responses(
        (
            status = OK,
            body = SessionDto
        ),
        (
            status = UNAUTHORIZED,
            body = JsonError
        ),
        BadRequestResponse
    ),
)]
pub async fn log_in<M: ModulesExt>(
    modules: State<M>,
    Json(source): Json<CreateSessionDto>,
) -> Result<impl IntoResponse, ApiError> {
    let credentials = source.parse()?;

    let user = modules.user_usecase().authorize(credentials).await?;

    modules
        .session_usecase()
        .create(SessionEntity::from(&user))
        .await?
        .conv::<SessionDto>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
