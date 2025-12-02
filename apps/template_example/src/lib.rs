use std::net::{IpAddr, Ipv6Addr};

use lib::env_vars_config::env_vars_config;

pub mod bootstrappers;
mod modules;

pub use modules::Modules;

env_vars_config! {
    SERVER_HOST: IpAddr = Ipv6Addr::UNSPECIFIED,
    SERVER_PORT: u16 = 8080_u16,
    POSTGRES_HOST: String = "localhost",
    POSTGRES_PORT: u16 = 5432_u16,
    POSTGRES_USER: String = "postgres",
    POSTGRES_PASSWORD: String = "postgres",
    POSTGRES_DATABASE: String = "template_example",
    OTEL_ENDPOINT: String = "http://localhost:4317",
    OTEL_SERVICE_NAMESPACE: String = "template_example",
    OTEL_SERVICE_NAME: String = "monolyth",
    JWT_SECRET: String = "changeme",
    DEPLOY_DOMAIN: String = "localhost",
}
