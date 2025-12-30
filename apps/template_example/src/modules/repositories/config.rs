use fromenv::FromEnv;
use lib::mobc_sqlx::sqlx::postgres::PgConnectOptions;

#[derive(FromEnv)]
pub struct RepositoriesConfig {
    #[env(nested)]
    pub postgres: PostgresConfig,
}

#[derive(FromEnv)]
#[env(prefix = "POSTGRES_")]
pub struct PostgresConfig {
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
