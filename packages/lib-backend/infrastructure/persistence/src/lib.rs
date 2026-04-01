use application::di::Has;
use mobc::{Connection, Manager, Pool};
#[cfg(feature = "redis")]
pub use redis;

pub mod entity;
pub mod repository;

#[cfg(feature = "mobc-sqlx")]
pub mod mobc_sqlx;

#[doc(hidden)]
pub use {derive_where::derive_where, pastey};

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
