use lib::{
    infrastructure::persistence::{
        mobc_sqlx::migrate_all, sqlx::SqlxPool,
    },
    mobc_sqlx::{SqlxConnectionManager, mobc::Pool},
    tap::Pipe as _,
};
use sqlx::{
    Postgres, migrate::Migrator, postgres::PgConnectOptions,
};

use crate::features::user::infrastructure::persistence::postgres::USER_POSTGRES_MIGRATOR;

pub(super) use self::config::PostgresConfig;
use super::RepositoriesModule;

mod config;

static POSTGRES_MIGRATORS: &[&Migrator] =
    &[&USER_POSTGRES_MIGRATOR];

impl RepositoriesModule {
    pub(super) async fn setup_postgres(
        config: &PostgresConfig,
    ) -> SqlxPool<Postgres> {
        let postgres = PgConnectOptions::from(config)
            .pipe(SqlxConnectionManager::new)
            .pipe(Pool::new);

        if config.run_migrator {
            migrate_all(&postgres, POSTGRES_MIGRATORS)
                .await;
        }

        postgres
    }
}
