use std::{str::FromStr as _, sync::OnceLock};

use application::repository::session::SessionRepositoryImpl;
use domain::session::{Session, entity::SessionEntity};
use entrait::entrait;
use lib::{
    anyhow::{Context as _, Result},
    application::di::Has,
    async_trait,
    infrastructure::persistence::redis::{Namespace, RedisPool},
    instrument_all,
    tap::Pipe as _,
    uuid::Uuid,
};
use redis::AsyncTypedCommands as _;

use crate::repository::RedisRepositoryImpl;

#[entrait(HasSessionNamespace)]
fn session_namespace<App>(app: &App) -> &Namespace
where
    App: Has<Namespace>,
{
    static NAMESPACE: OnceLock<Namespace> = OnceLock::new();
    NAMESPACE.get_or_init(|| app.get_dependency().nest("session"))
}

#[entrait(ref)]
#[async_trait]
#[instrument_all]
impl SessionRepositoryImpl for RedisRepositoryImpl {
    async fn save_session<App>(app: &App, source: Session) -> Result<Session>
    where
        App: Has<RedisPool> + HasSessionNamespace,
    {
        let mut connection = app.get_dependency().get().await?;

        let (entity_type, entity_id) = source.entity.as_tuple();

        connection
            .set_ex(
                app.session_namespace()
                    .nest(entity_type)
                    .key(&entity_id.to_string()),
                source.id.to_string(),
                Session::LIFETIME
                    .try_into()
                    .context("while converting session lifetime")?,
            )
            .await?;

        Ok(source)
    }

    async fn find_session_by_entity<App>(
        app: &App,
        entity: SessionEntity,
    ) -> Result<Option<Session>>
    where
        App: Has<RedisPool> + HasSessionNamespace,
    {
        let mut connection = app.get_dependency().get().await?;

        let (entity_type, entity_id) = entity.as_tuple();

        connection
            .get(
                app.session_namespace()
                    .nest(entity_type)
                    .key(&entity_id.to_string()),
            )
            .await?
            .map(|id| Uuid::from_str(&id))
            .transpose()
            .context("while parsing session ID from cache")?
            .map(|id| Session {
                id: id.into(),
                entity,
            })
            .pipe(Ok)
    }
}
