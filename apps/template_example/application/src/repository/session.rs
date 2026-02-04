use domain::session::{Session, entity::SessionEntity};
use lib::{anyhow::Result, async_trait};

#[async_trait]
pub trait SessionRepository {
    async fn save(&self, source: Session) -> Result<Session>;

    async fn find_by_entity(
        &self,
        entity: SessionEntity,
    ) -> Result<Option<Session>>;
}
