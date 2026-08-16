use lib::{
    infrastructure::persistence::{SqlxPool, mobc_sqlx::migrate_all},
    mobc_sqlx::SqlxConnectionManager,
    tap::Pipe as _,
};
use sqlx::{Postgres, migrate::Migrator, postgres::PgConnectOptions};

pub(super) use self::config::PostgresConfig;
use super::RepositoriesModule;
use crate::features::user::infrastructure::persistence::postgres::USER_POSTGRES_MIGRATOR;

mod config;

static POSTGRES_MIGRATORS: &[&Migrator] = &[&USER_POSTGRES_MIGRATOR];

impl RepositoriesModule {
    pub(super) async fn setup_postgres(
        config: &PostgresConfig,
    ) -> SqlxPool<Postgres> {
        let postgres = PgConnectOptions::from(config)
            .pipe(SqlxConnectionManager::new)
            .pipe(SqlxPool::new);

        if config.run_migrator {
            migrate_all(&postgres, POSTGRES_MIGRATORS).await;
        }

        postgres
    }
}
