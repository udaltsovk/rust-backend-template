use domain::session::{Session, entity::SessionEntity};
use lib::{async_trait, instrument_all};

use crate::{
    repository::{RepositoriesModuleExt, session::SessionRepository as _},
    service::{ServicesModuleExt, token::TokenService as _},
    usecase::{
        UseCase,
        session::{
            SessionUseCase,
            error::{SessionUseCaseError, SessionUseCaseResult},
        },
    },
};

#[async_trait]
#[instrument_all]
impl<R, S> SessionUseCase<R, S> for UseCase<R, S, Session>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn create(
        &self,
        entity: SessionEntity,
    ) -> SessionUseCaseResult<R, S, String> {
        let session = {
            use SessionEntity as SE;
            match entity {
                SE::User(id) => Session::new_for_user(id),
            }
        };

        let session = self
            .repositories
            .session_repository()
            .save(session)
            .await
            .map_err(R::Error::from)
            .map_err(SessionUseCaseError::Repository)?;

        self.services
            .token_service()
            .generate(session)
            .map_err(S::Error::from)
            .map_err(SessionUseCaseError::Service)
    }
}
