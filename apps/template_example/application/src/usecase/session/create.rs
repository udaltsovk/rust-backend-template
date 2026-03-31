use domain::session::{Session, entity::SessionEntity};
use entrait::entrait;
use lib::redact::Secret;
use tracing::instrument;

use crate::{
    repository::session::SessionRepository,
    service::token::TokenService,
    usecase::session::error::{SessionUseCaseError, SessionUseCaseResult},
};

#[entrait(pub CreateSessionUsecase)]
#[instrument(skip(app))]
async fn create_session<App>(
    app: &App,
    entity: SessionEntity,
) -> SessionUseCaseResult<Secret<String>>
where
    App: SessionRepository + TokenService,
{
    let session = {
        use SessionEntity as SE;
        match entity {
            SE::User(id) => Session::new_for_user(id),
        }
    };

    let session = app
        .save_session(session)
        .await
        .map_err(SessionUseCaseError::Infrastructure)?;

    app.generate_token(session)
        .map_err(SessionUseCaseError::Infrastructure)
}
