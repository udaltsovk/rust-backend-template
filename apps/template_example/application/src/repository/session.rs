use std::fmt::Debug;

use domain::session::{Session, entity::SessionEntity};
use lib::async_trait;

#[async_trait]
pub trait SessionRepository {
    type AdapterError: Debug + Send + Sync;

    async fn save(
        &self,
        source: Session,
    ) -> Result<Session, Self::AdapterError>;

    async fn find_by_entity(
        &self,
        entity: SessionEntity,
    ) -> Result<Option<Session>, Self::AdapterError>;
}
