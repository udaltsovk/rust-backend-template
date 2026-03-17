use entrait::entrait;
use mobc::{Manager, Pool};
// #[cfg(feature = "postgres")]
// pub use postgres;
#[cfg(feature = "redis")]
pub use redis;

pub mod entity;
pub mod repository;

#[cfg(feature = "mobc-sqlx")]
pub mod mobc_sqlx;

#[doc(hidden)]
pub use {derive_where::derive_where, pastey};

#[entrait(pub HasPool)]
fn pool<M>(pool: &Pool<M>) -> &Pool<M>
where
    M: Manager,
{
    pool
}
