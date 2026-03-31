use domain::session::Session;
use entrait::entrait;
use lib::redact::Secret;
use tracing::instrument;

use crate::{
    repository::session::SessionRepository,
    service::token::TokenService,
    usecase::session::error::{SessionUseCaseError, SessionUseCaseResult},
};

#[entrait(pub GetSessionFromTokenUsecase)]
#[instrument(skip(app))]
async fn get_session_from_token<App>(
    app: &App,
    token: Secret<&str>,
) -> SessionUseCaseResult<Session>
where
    App: SessionRepository + TokenService,
{
    let session = app.parse_token(token)?;

    app.find_session_by_entity(session.entity)
        .await?
        .is_some_and(|ses| session == ses)
        .ok_or(SessionUseCaseError::NotFound(session.id))?;

    Ok(session)
}
