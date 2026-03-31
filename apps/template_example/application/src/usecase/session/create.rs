use domain::session::{Session, entity::SessionEntity};
use entrait::entrait;
use lib::{redact::Secret, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::session::SessionRepository, service::token::TokenService,
    usecase::session::error::SessionUseCaseResult,
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

    let session = app.save_session(session).await?;

    app.generate_token(session)?.pipe(Ok)
}
