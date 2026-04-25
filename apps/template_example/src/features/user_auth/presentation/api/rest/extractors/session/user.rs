use axum::{
    RequestPartsExt as _, extract::FromRequestParts,
    http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use lib::{domain::Id, redact::Secret, tap::Pipe as _};

use crate::{
    features::{
        user::domain::User,
        user_auth::{
            application::usecase::session::GetSessionFromTokenUsecase,
            domain::session::{
                Session, entity::SessionEntity,
            },
            presentation::api::rest::errors::AuthError,
        },
    },
    shared::presentation::api::rest::ApiError,
};

#[derive(Debug)]
pub struct UserSession {
    pub id: Id<Session>,
    pub user_id: Id<User>,
}

impl<App> FromRequestParts<App> for UserSession
where
    App: Sync + GetSessionFromTokenUsecase,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        app: &App,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let session = app
            .get_session_from_token(Secret::new(
                bearer.token(),
            ))
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        #[expect(
            unreachable_patterns,
            reason = "other session entities may be added \
                      in the future"
        )]
        match session.entity {
            SessionEntity::User(user_id) => Self {
                id: session.id,
                user_id,
            }
            .pipe(Ok),
            _ => AuthError::InvalidToken
                .pipe(Err)
                .map_err(Self::Rejection::from),
        }
    }
}
