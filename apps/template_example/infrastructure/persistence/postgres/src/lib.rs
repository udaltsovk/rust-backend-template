use lib::infrastructure::persistence::postgres::Postgres;

pub mod entity;
pub mod repository;

pub async fn migrate(postgres: &Postgres) {
    postgres
        .migrate(sqlx::migrate!("./migrations"))
        .await
        .expect("failed to run migrations");
}
