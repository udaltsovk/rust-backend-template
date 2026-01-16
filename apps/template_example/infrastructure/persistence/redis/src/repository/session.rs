use std::{str::FromStr as _, sync::LazyLock};

use application::repository::session::SessionRepository;
use domain::session::{Session, entity::SessionEntity};
use lib::{
    async_trait,
    infrastructure::persistence::redis::{Namespace, error::RedisAdapterError},
    instrument_all,
    tap::Pipe as _,
    uuid::Uuid,
};
use redis::AsyncTypedCommands as _;

use crate::repository::{META_NAMESPACE, RedisRepositoryImpl};

static NAMESPACE: LazyLock<Namespace> =
    LazyLock::new(|| META_NAMESPACE.nest("session"));

#[async_trait]
#[instrument_all]
impl SessionRepository for RedisRepositoryImpl<Session> {
    type AdapterError = RedisAdapterError;

    async fn save(
        &self,
        source: Session,
    ) -> Result<Session, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        let (entity_type, entity_id) = source.entity.as_tuple();

        connection
            .set_ex(
                NAMESPACE.nest(entity_type).key(&entity_id.to_string()),
                source.id.to_string(),
                Session::LIFETIME
                    .try_into()
                    .expect("lifetime convertion to not fail"),
            )
            .await?;

        Ok(source)
    }

    async fn find_by_entity(
        &self,
        entity: SessionEntity,
    ) -> Result<Option<Session>, Self::AdapterError> {
        let mut connection = self.pool.get().await?;

        let (entity_type, entity_id) = entity.as_tuple();

        connection
            .get(NAMESPACE.nest(entity_type).key(&entity_id.to_string()))
            .await?
            .map(|id| Uuid::from_str(&id))
            .transpose()
            .expect("session ID from cache should be valid")
            .map(|id| Session {
                id: id.into(),
                entity,
            })
            .pipe(Ok)
    }
}
