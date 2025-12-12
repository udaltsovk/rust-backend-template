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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use mobc_sqlx::{SqlxConnectionManager, sqlx::Postgres};

    use super::*;

    #[tokio::test]
    #[should_panic(expected = "failed to run migrations")]
    async fn migrate_panics_without_db() {
        struct TempDir(PathBuf);
        impl Drop for TempDir {
            #[expect(
                clippy::print_stderr,
                reason = "We want to print errors in drop implementation for tests"
            )]
            fn drop(&mut self) {
                if let Err(e) = std::fs::remove_dir_all(&self.0) {
                    eprintln!("Failed to remove temp dir: {e}");
                }
            }
        }

        let temp_path = std::env::temp_dir().join(format!(
            "migrations_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_nanos()
        ));
        std::fs::create_dir_all(&temp_path).expect("failed to create temp dir");
        let _temp_dir = TempDir(temp_path.clone());

        // Create a dummy migration file
        let migration_file = temp_path.join("20230101000000_test.sql");
        std::fs::write(&migration_file, "SELECT 1;")
            .expect("failed to write migration file");

        let manager = SqlxConnectionManager::<Postgres>::new(
            "postgres://user:pass@localhost:5432/db",
        );
        let pool = Pool::builder().max_open(1).build(manager);

        let migrator =
            Migrator::new(temp_path).await.expect("migrator created");

        migrator.migrate(&pool).await;
    }
}
