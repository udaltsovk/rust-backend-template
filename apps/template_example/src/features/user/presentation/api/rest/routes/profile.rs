use axum::{
    extract::State, http::StatusCode,
    response::IntoResponse,
};
use lib::{
    presentation::api::rest::{
        errors::JsonError, response::ResponseExt as _,
    },
    tap::{Conv as _, Pipe as _},
};
use tracing::instrument;

use crate::{
    features::{
        user_auth::presentation::api::rest::extractors::session::UserSession,
        user::{
            application::usecase::GetUserByIdUsecase,
            presentation::api::rest::dto::UserDto,
        },
    },
    shared::presentation::api::rest::{
        ApiError, B2C_TAG, extractors::Json,
    },
};

#[utoipa::path(
    get,
    path = "/profile",
    tag = B2C_TAG,
    security(
        ("user" = []),
    ),
    responses(
        (status = OK, body = UserDto),
        (status = UNAUTHORIZED, body = JsonError),
    ),
)]
#[instrument(skip(app))]
pub async fn get_profile<App>(
    app: State<App>,
    user_session: UserSession,
) -> Result<impl IntoResponse, ApiError>
where
    App: GetUserByIdUsecase,
{
    app.get_user_by_id(user_session.user_id)
        .await
        .map_err(ApiError::from)?
        .conv::<UserDto>()
        .pipe(Json)
        .into_response()
        .with_status(StatusCode::OK)
        .pipe(Ok)
}
