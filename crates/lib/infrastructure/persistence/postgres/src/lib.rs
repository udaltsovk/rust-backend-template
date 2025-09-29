use std::ops::Deref;

use sqlx::{Pool, Postgres as PostgreSQL, postgres::PgPoolOptions};

pub mod entity;
pub mod error;

#[derive(Clone)]
pub struct Postgres(Pool<PostgreSQL>);

impl Postgres {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(database_url)
            .await
            .expect("Cannot connect to the database. Please check your configuration.");
        Self(pool)
    }
}

impl Deref for Postgres {
    type Target = Pool<PostgreSQL>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
