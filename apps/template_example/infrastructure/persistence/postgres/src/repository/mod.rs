use lib::infrastructure::persistence::repository_impl_struct;
use mobc_sqlx::{SqlxConnectionManager, mobc};
use sqlx::Postgres;

mod user;

repository_impl_struct!(Postgres, SqlxConnectionManager<Postgres>);
