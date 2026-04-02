use mobc_sqlx::{
    SqlxConnectionManager, SqlxMigrationExt as _,
    mobc::{Pool, async_trait},
    sqlx::{
        Database,
        migrate::{Migrate, Migrator},
    },
};

#[async_trait]
pub trait MigratorExt {
    async fn migrate<DB>(&self, pool: &Pool<SqlxConnectionManager<DB>>)
    where
        DB: Database + Sync,
        <DB as Database>::Connection: Migrate;
}

#[async_trait]
impl MigratorExt for Migrator {
    async fn migrate<DB>(&self, pool: &Pool<SqlxConnectionManager<DB>>)
    where
        DB: Database + Sync,
        <DB as Database>::Connection: Migrate,
    {
        pool.migrate(&self).await.expect("failed to run migrations");
    }
}
