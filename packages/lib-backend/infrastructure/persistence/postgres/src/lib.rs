use std::ops::Deref;

#[doc(hidden)]
pub use derive_where::derive_where;
use sqlx::{
    Pool, Postgres as PostgreSQL, migrate::Migrator, postgres::PgPoolOptions,
};
use tap::Pipe as _;

pub mod error;
pub mod repository;

#[derive(Clone)]
pub struct Postgres(Pool<PostgreSQL>);

impl Postgres {
    pub async fn new(database_url: &str) -> Self {
        PgPoolOptions::new()
            .max_connections(8)
            .connect(database_url)
            .await
            .expect("Cannot connect to the database. Please check your configuration.")
            .pipe(Self)
    }

    pub async fn migrate(&self, migrator: Migrator) -> Result<(), sqlx::Error> {
        let mut transaction = self.begin().await?;

        migrator.run(&mut transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
}

impl Deref for Postgres {
    type Target = Pool<PostgreSQL>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
