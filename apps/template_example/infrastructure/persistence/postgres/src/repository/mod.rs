use lib::infrastructure::persistence::repository_impl_struct;
use mobc_sqlx::SqlxConnectionManager;
use sqlx::Postgres;

mod client;

repository_impl_struct!(Postgres, SqlxConnectionManager<Postgres>);
