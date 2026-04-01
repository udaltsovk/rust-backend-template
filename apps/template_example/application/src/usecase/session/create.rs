use domain::session::{Session, entity::SessionEntity};
use entrait::entrait;
use lib::{redact::Secret, tap::Pipe as _};
use tracing::instrument;

use crate::{
    repository::session::SessionRepository, service::token::TokenService,
    usecase::session::SessionUseCaseResult,
};

#[entrait(pub CreateSessionUsecase)]
#[instrument(skip(deps))]
async fn create_session<Deps>(
    deps: &Deps,
    entity: SessionEntity,
) -> SessionUseCaseResult<Secret<String>>
where
    Deps: SessionRepository + TokenService,
{
    let session = {
        use SessionEntity as SE;
        match entity {
            SE::User(id) => Session::new_for_user(id),
        }
    };

    let session = SessionRepository::save_session(deps, session).await?;

    TokenService::generate_token(deps, session)?.pipe(Ok)
}
