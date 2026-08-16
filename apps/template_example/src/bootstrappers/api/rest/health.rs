use axum::Router;
use entrait::Impl;
use lib::presentation::api::rest::health::{live, ready};

use crate::modules::Modules;

pub const LIVE_PATH: &str = "/health/live";
pub const READY_PATH: &str = "/health/ready";

pub fn router() -> Router<Impl<Modules>> {
    Router::new()
        .route(LIVE_PATH, live())
        .route(READY_PATH, ready::<Modules>())
}

#[must_use]
pub fn is_health_route(path: &str) -> bool {
    matches!(path, LIVE_PATH | READY_PATH)
}
