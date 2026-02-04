use domain::session::{Session, entity::SessionEntity};
use lib::{async_trait, instrument_all, redact::Secret};

use crate::usecase::{
    UseCase,
    session::{
        SessionUseCase,
        error::{SessionUseCaseError, SessionUseCaseResult},
    },
};

#[async_trait]
#[instrument_all]
impl SessionUseCase for UseCase<Session> {
    async fn create(
        &self,
        entity: SessionEntity,
    ) -> SessionUseCaseResult<Secret<String>> {
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
            .map_err(SessionUseCaseError::Infrastructure)?;

        self.services
            .token_service()
            .generate(session)
            .map_err(SessionUseCaseError::Infrastructure)
    }

    async fn get_from_token(
        &self,
        token: Secret<&str>,
    ) -> SessionUseCaseResult<Session> {
        let session = self
            .services
            .token_service()
            .parse(token)
            .map_err(SessionUseCaseError::Infrastructure)?;

        self.repositories
            .session_repository()
            .find_by_entity(session.entity)
            .await
            .map_err(SessionUseCaseError::Infrastructure)?
            .is_some_and(|ses| session == ses)
            .ok_or(SessionUseCaseError::NotFound(session.id))?;

        Ok(session)
    }
}
