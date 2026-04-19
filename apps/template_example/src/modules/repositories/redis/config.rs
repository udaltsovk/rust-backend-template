use std::fmt::Write as _;

use fromenv::FromEnv;

#[derive(FromEnv, Clone)]
#[env(prefix = "REDIS_")]
pub struct RedisConfig {
    pub host: String,
    pub port: Option<u16>,
    pub user: Option<String>,
    pub password: Option<String>,
    pub database: Option<String>,
    #[env(default = "template_example")]
    pub service_namespace: String,
    #[env(default = "monolyth")]
    pub service_name: String,
}

impl From<&RedisConfig> for redis::Client {
    fn from(config: &RedisConfig) -> Self {
        let url = try {
            let mut url = "redis://".to_string();

            if let Some(username) = &config.user {
                write!(url, "{username}")?;

                if let Some(password) = &config.password {
                    write!(url, ":{password}")?;
                }

                write!(url, "@")?;
            }

            write!(url, "{}", &config.host)?;

            if let Some(port) = &config.port {
                write!(url, ":{port}")?;
            }

            if let Some(database) = &config.database {
                write!(url, "/{database}")?;
            }

            url
        }
        .expect(
            "url formatting should finish successfully",
        );

        Self::open(url)
            .expect("redis client should open successfully")
    }
}
