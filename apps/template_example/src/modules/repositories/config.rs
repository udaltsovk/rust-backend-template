use better_config::{EnvConfig, env};
use lib::mobc_sqlx::sqlx::postgres::PgConnectOptions;

#[env(EnvConfig)]
pub struct RepositoriesConfig {
    #[env]
    pub postgres: PostgresConfig,
}

#[env(EnvConfig(prefix = "POSTGRES_"))]
pub struct PostgresConfig {
    #[conf(default = "postgres")]
    pub user: String,
    #[conf(default = "postgres")]
    pub password: String,
    #[conf(default = "localhost")]
    pub host: String,
    #[conf(default = "5432")]
    pub port: u16,
    #[conf(default = "template_example")]
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
