use lib::utoipa_axum::{router::OpenApiRouter, routes};

use crate::ModulesExt;

pub mod auth;
pub mod profile;

pub const B2C_TAG: &str = "B2C";

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new()
        .routes(routes!(auth::sign_up::<M>))
        .routes(routes!(auth::log_in::<M>))
        .routes(routes!(profile::get_profile::<M>))
}
