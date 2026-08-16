use lib::{
    application::{
        di::Has as _,
        health::{HealthCheck as _, Readiness, ReadinessReport},
    },
    infrastructure::persistence::{RedisPool, SqlxPool},
    mobc_sqlx::sqlx::Postgres,
    presentation::api::rest::health::ReadinessCheck,
};

pub use self::config::ModulesConfig;
use self::{repositories::RepositoriesModule, services::ServicesModule};

mod config;
mod repositories;
mod services;

#[derive(Clone)]
pub struct Modules {
    config: &'static ModulesConfig,
    repositories: RepositoriesModule,
    services: ServicesModule,
}

impl Modules {
    pub async fn init(config: &'static ModulesConfig) -> Self {
        Self {
            config,
            repositories: RepositoriesModule::new(&config.repositories).await,
            services: ServicesModule::new(&config.services),
        }
    }
}

impl ReadinessCheck for Modules {
    async fn readiness(&self) -> ReadinessReport {
        let postgres: &SqlxPool<Postgres> = self.get_dependency();
        let redis: &RedisPool = self.get_dependency();

        Readiness::new()
            .check("postgres", postgres.health_check())
            .check("redis", redis.health_check())
            .run()
            .await
    }
}
