#![allow(
    clippy::non_minimal_cfg,
    reason = "we'll add more crates and more features"
)]

#[cfg(feature = "application")]
pub use application;
#[cfg(any(
    feature = "application",
    feature = "bootstrap",
    feature = "infrastructure-persistence",
    feature = "infrastructure-services",
))]
pub use async_trait::async_trait;
#[cfg(all(feature = "bootstrap", feature = "presentation-api-rest"))]
pub use axum;
#[cfg(all(
    feature = "bootstrap-instrumentation-opentelemetry",
    feature = "presentation-api-rest"
))]
pub use axum_otel_metrics;
#[cfg(all(
    feature = "bootstrap-instrumentation-opentelemetry",
    feature = "presentation-api-rest"
))]
pub use axum_tracing_opentelemetry;
#[cfg(feature = "bootstrap")]
pub use bootstrap;
#[cfg(any(feature = "infrastructure-services",))]
pub use chrono;
#[cfg(feature = "domain")]
pub use domain;
#[cfg(feature = "bootstrap")]
pub use env_vars_config;
#[cfg(feature = "infrastructure")]
pub use infrastructure;
#[cfg(feature = "macros")]
pub use macros::*;
#[cfg(feature = "bootstrap")]
pub use mobc_sqlx;
#[cfg(any(
    feature = "infrastructure-persistence",
    feature = "infrastructure-services",
    feature = "presentation-api",
))]
pub use model_mapper;
#[cfg(feature = "presentation")]
pub use presentation;
#[cfg(any(
    feature = "application",
    feature = "infrastructure-persistence",
    feature = "infrastructure-services",
    feature = "presentation",
))]
pub use tap;
#[cfg(feature = "bootstrap")]
pub use tower;
#[cfg(feature = "bootstrap")]
pub use tower_http;
#[cfg(any(feature = "presentation-api-rest",))]
pub use utoipa_axum;
#[cfg(any(
    feature = "domain",
    feature = "infrastructure-persistence",
    feature = "infrastructure-services",
    feature = "presentation-api",
))]
pub use uuid;
