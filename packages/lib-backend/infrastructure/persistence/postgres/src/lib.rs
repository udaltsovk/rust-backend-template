use std::ops::Deref;

use sqlx::{Pool, Postgres as PostgreSQL, postgres::PgPoolOptions};
use tap::Pipe as _;

pub mod entity;
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
}

impl Deref for Postgres {
    type Target = Pool<PostgreSQL>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
