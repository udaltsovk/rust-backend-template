use sqlx::migrate::Migrator;

pub mod entity;
pub mod repository;

pub static POSTGRES_MIGRATOR: Migrator = sqlx::migrate!("./migrations");
