use std::ops::Deref;

use sqlx::{Pool, Postgres, postgres::PgPoolOptions};

pub mod entity;
pub mod error;
pub mod module;
pub mod repository;

#[derive(Clone)]
pub struct Db(Pool<Postgres>);
impl Db {
    pub async fn new(database_url: &str) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(8)
            .connect(database_url)
            .await
            .expect("Cannot connect to the database. Please check your configuration.");
        Self(pool)
    }
}
impl Deref for Db {
    type Target = Pool<Postgres>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
