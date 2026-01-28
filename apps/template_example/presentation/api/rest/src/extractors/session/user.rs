use application::usecase::session::SessionUseCase as _;
use axum::{
    RequestPartsExt as _, extract::FromRequestParts, http::request::Parts,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use domain::{
    session::{Session, entity::SessionEntity},
    user::User,
};
use lib::{domain::Id, tap::Pipe as _};
use redact::Secret;

use crate::{ApiError, ModulesExt, errors::AuthError};

pub struct UserSession {
    pub id: Id<Session>,
    pub user_id: Id<User>,
}

impl<M> FromRequestParts<M> for UserSession
where
    M: ModulesExt,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &M,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::InvalidToken)?;

        let session = state
            .session_usecase()
            .get_from_token(Secret::new(bearer.token()))
            .await?;

        #[expect(
            unreachable_patterns,
            reason = "other session entities may be added in the future"
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
