use fromenv::FromEnv;
use sqlx::postgres::PgConnectOptions;

#[derive(FromEnv, Clone)]
#[env(prefix = "POSTGRES_")]
pub struct PostgresConfig {
    #[env(default = "true")]
    pub run_migrator: bool,
    pub user: String,
    pub password: String,
    pub host: String,
    #[env(default = "5432")]
    pub port: u16,
    pub database: String,
}

impl From<&PostgresConfig> for PgConnectOptions {
    fn from(config: &PostgresConfig) -> Self {
        Self::new()
            .username(&config.user)
            .password(&config.password)
            .host(&config.host)
            .port(config.port)
            .database(&config.database)
    }
}
