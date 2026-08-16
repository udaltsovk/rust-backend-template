use std::ops::Deref;

use application::{
    di::Has,
    health::{CheckResult, HealthCheck},
};
use mobc::{Connection, Manager, Pool as MobcPool};
#[cfg(feature = "redis")]
pub use redis;
#[cfg(feature = "sqlx")]
pub use sqlx;
use tap::Pipe as _;

pub mod entity;
pub mod repository;

#[cfg(feature = "mobc-sqlx")]
pub mod mobc_sqlx;

#[doc(hidden)]
pub use {derive_where::derive_where, pastey};

pub struct Pool<M: Manager>(MobcPool<M>);

impl<M: Manager> Pool<M> {
    #[must_use]
    pub fn new(manager: M) -> Self {
        manager.pipe(MobcPool::new).pipe(Self)
    }
}

impl<M: Manager> Clone for Pool<M> {
    fn clone(&self) -> Self {
        self.0.clone().pipe(Self)
    }
}

impl<M: Manager> Deref for Pool<M> {
    type Target = MobcPool<M>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<M> HealthCheck for Pool<M>
where
    M: Manager,
    mobc::Error<M::Error>: std::fmt::Display,
{
    fn health_check(
        &self,
    ) -> impl Future<Output = CheckResult> + Send + 'static {
        let pool = self.0.clone();

        async move {
            pool.get()
                .await
                .map(drop)
                .map_err(|error| error.to_string())
        }
    }
}

#[cfg(feature = "sqlx")]
pub type SqlxPool<DB> = Pool<sqlx::SqlxConnectionManager<DB>>;

#[cfg(feature = "redis")]
pub type RedisPool = Pool<redis::RedisConnectionManager>;

pub trait HasPoolExt<M>
where
    M: Manager,
{
    fn get_connection(
        &self,
    ) -> impl Future<Output = Result<Connection<M>, mobc::Error<M::Error>>>;
}

impl<D, M> HasPoolExt<M> for D
where
    D: Has<Pool<M>>,
    M: Manager,
{
    fn get_connection(
        &self,
    ) -> impl Future<
        Output = Result<Connection<M>, mobc::Error<<M as Manager>::Error>>,
    > {
        self.get_dependency().get()
    }
}
