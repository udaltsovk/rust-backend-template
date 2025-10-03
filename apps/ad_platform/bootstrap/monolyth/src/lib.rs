use std::{
    net::{IpAddr, SocketAddr},
    str::FromStr as _,
};

use env_vars_config::env_vars_config;

pub mod bootstrappers;
mod modules;

pub use modules::Modules;

env_vars_config! {
    SERVER_ADDRESS: IpAddr = IpAddr::from_str("0.0.0.0").expect("a valid IP address"),
    SERVER_PORT: u16 = 8080u16,
    POSTGRES_USER: String = "postgres",
    POSTGRES_PASSWORD: String = "postgres",
    POSTGRES_ADDRESS: String = "localhost",
    POSTGRES_PORT: u16 = 5432u16,
    POSTGRES_DATABASE: String = "ad_platform",
    OTEL_ENDPOINT: String = "http://localhost:4317",
    OTEL_SERVICE_NAME: String = "ad_platform-monolyth",
    PROMETHEUS_ADDRESS: SocketAddr = SocketAddr::from_str("0.0.0.0:8081").expect("a valid socket address"),
    JWT_SECRET: String = "changeme",
}
