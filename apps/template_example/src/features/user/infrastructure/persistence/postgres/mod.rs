use lib::migrate;
use sqlx::migrate::Migrator;

pub mod entity;
pub mod repository;

pub(crate) static USER_POSTGRES_MIGRATOR: Migrator =
    migrate!();
