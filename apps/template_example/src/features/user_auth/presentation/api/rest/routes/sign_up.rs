use axum::{
    extract::State, http::StatusCode,
    response::IntoResponse,
};
use lib::{
    presentation::api::rest::{
        errors::JsonError, response::ResponseExt as _,
        validation::parseable::Parseable as _,
    },
    tap::{Conv as _, Pipe as _},
};
use tracing::instrument;

use crate::{
    features::{
        user::{
            application::usecase::CreateUserUsecase,
            presentation::api::rest::dto::CreateUserDto,
        },
        user_auth::{
            application::usecase::session::CreateSessionUsecase,
            presentation::api::rest::dto::session::SessionDto,
        },
    },
    shared::presentation::api::rest::{
        ApiError, B2C_TAG,
        errors::{
            BadRequestResponse, ValidationFailedResponse,
        },
        extractors::Json,
    },
};

/// Регистрация нового пользователя
///
/// Регистрирует нового пользователя и возвращает токен
/// доступа.
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

    app.create_session(user.into())
        .await?
        .conv::<SessionDto>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
