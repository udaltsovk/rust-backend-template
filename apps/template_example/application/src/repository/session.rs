use domain::session::{Session, entity::SessionEntity};
use entrait::entrait;
use lib::{anyhow::Result, async_trait};

#[entrait(
    SessionRepositoryImpl,
    delegate_by=DelegateSessionRepository
)]
#[async_trait]
pub trait SessionRepository {
    async fn save_session(&self, source: Session) -> Result<Session>;

    async fn find_session_by_entity(
        &self,
        entity: SessionEntity,
    ) -> Result<Option<Session>>;
}
