use application::usecase::{
    session::CreateSessionUsecase,
    user::{AuthorizeUserUsecase, CreateUserUsecase},
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
use tracing::instrument;

use crate::{
    ApiError,
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
#[instrument(skip(app))]
pub async fn sign_up<App>(
    app: State<App>,
    Json(source): Json<CreateUserDto>,
) -> Result<impl IntoResponse, ApiError>
where
    App: CreateUserUsecase + CreateSessionUsecase,
{
    let user = source.parse()?;

    let user = app.create_user(user).await?;

    app.create_session(SessionEntity::from(&user))
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
#[instrument(skip(app))]
pub async fn log_in<App>(
    app: State<App>,
    Json(source): Json<CreateSessionDto>,
) -> Result<impl IntoResponse, ApiError>
where
    App: AuthorizeUserUsecase + CreateSessionUsecase,
{
    let credentials = source.parse()?;

    let user = app.authorize_user(credentials).await?;

    app.create_session(SessionEntity::from(&user))
        .await?
        .conv::<SessionDto>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
