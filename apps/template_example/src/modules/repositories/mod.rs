use std::sync::OnceLock;

use lib::{
    application::impl_has,
    bootstrap::impl_repositories,
    infrastructure::persistence::{
        redis::{Namespace, RedisPool},
        sqlx::SqlxPool,
    },
    mobc_sqlx::sqlx::Postgres,
};

pub use self::config::RepositoriesConfig;
use super::Modules;
use crate::{
    features::{
        user::application::repository::DelegateUserRepository,
        user_auth::application::repository::session::DelegateSessionRepository,
    },
    shared::infrastructure::persistence::{
        PostgresRepositoryImpl, RedisRepositoryImpl,
    },
};

mod config;
mod postgres;
mod redis;

#[derive(Clone)]
pub struct RepositoriesModule {
    postgres: SqlxPool<Postgres>,
    redis: RedisPool,
}

impl RepositoriesModule {
    pub(crate) async fn new(
        config: &RepositoriesConfig,
    ) -> Self {
        Self {
            postgres: Self::setup_postgres(
                &config.postgres,
            )
            .await,
            redis: Self::setup_redis(&config.redis),
        }
    }
}

impl_has! {
    struct: Modules,
    SqlxPool<Postgres>: |s| &s.repositories.postgres,
    RedisPool: |s| &s.repositories.redis,
    Namespace: |s| {
        static NAMESPACE: OnceLock<Namespace> = OnceLock::new();
        NAMESPACE.get_or_init(|| {
            Namespace::new(&s.config.repositories.redis.service_namespace)
                .nest(&s.config.repositories.redis.service_name)
        })
    }
}

impl_repositories! {
    struct: Modules,
    DelegateUserRepository: PostgresRepositoryImpl,
    DelegateSessionRepository: RedisRepositoryImpl,
}
