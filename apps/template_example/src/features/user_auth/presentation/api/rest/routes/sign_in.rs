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
        user::application::usecase::AuthorizeUserUsecase,
        user_auth::{
            application::usecase::session::CreateSessionUsecase,
            domain::session::entity::SessionEntity,
            presentation::api::rest::dto::session::{
                CreateSessionDto, SessionDto,
            },
        },
    },
    shared::presentation::api::rest::{
        ApiError, B2C_TAG, errors::BadRequestResponse,
        extractors::Json,
    },
};

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
pub async fn sign_in<App>(
    app: State<App>,
    Json(source): Json<CreateSessionDto>,
) -> Result<impl IntoResponse, ApiError>
where
    App: AuthorizeUserUsecase + CreateSessionUsecase,
{
    source
        .parse()?
        .pipe(|credentials| app.authorize_user(credentials))
        .await?
        .conv::<SessionEntity>()
        .pipe(|entity| app.create_session(entity))
        .await?
        .conv::<SessionDto>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
