use std::{str::FromStr as _, sync::LazyLock};

use application::repository::session::SessionRepositoryImpl;
use domain::session::{Session, entity::SessionEntity};
use entrait::entrait;
use lib::{
    anyhow::Result, application::di::Has, async_trait,
    infrastructure::persistence::redis::Namespace, instrument_all,
    tap::Pipe as _, uuid::Uuid,
};
use mobc_redis::{RedisConnectionManager, mobc::Pool};
use redis::AsyncTypedCommands as _;

use crate::repository::{META_NAMESPACE, RedisRepositoryImpl};

static NAMESPACE: LazyLock<Namespace> =
    LazyLock::new(|| META_NAMESPACE.nest("session"));

#[entrait(ref)]
#[async_trait]
#[instrument_all]
impl SessionRepositoryImpl for RedisRepositoryImpl {
    async fn save_session<App>(app: &App, source: Session) -> Result<Session>
    where
        App: Has<Pool<RedisConnectionManager>>,
    {
        let mut connection = app.get_dependency().get().await?;

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

    async fn find_session_by_entity<App>(
        app: &App,
        entity: SessionEntity,
    ) -> Result<Option<Session>>
    where
        App: Has<Pool<RedisConnectionManager>>,
    {
        let mut connection = app.get_dependency().get().await?;

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
