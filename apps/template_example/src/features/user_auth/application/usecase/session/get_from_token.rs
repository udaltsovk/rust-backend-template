use entrait::entrait;
use lib::redact::Secret;
use tracing::instrument;

use super::{SessionUseCaseError, SessionUseCaseResult};
use crate::features::user_auth::{
    application::{
        repository::session::SessionRepository,
        service::token::TokenService,
    },
    domain::session::Session,
};

#[entrait(pub GetSessionFromTokenUsecase)]
#[instrument(skip(deps))]
async fn get_session_from_token<Deps>(
    deps: &Deps,
    token: Secret<&str>,
) -> SessionUseCaseResult<Session>
where
    Deps: SessionRepository + TokenService,
{
    let session = TokenService::parse_token(deps, token)?;

    SessionRepository::find_session_by_entity(
        deps,
        session.entity,
    )
    .await?
    .is_some_and(|ses| session == ses)
    .ok_or(SessionUseCaseError::NotFound(session.id))?;

    Ok(session)
}
