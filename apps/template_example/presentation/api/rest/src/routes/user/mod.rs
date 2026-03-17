use application::Application;
use lib::utoipa_axum::{router::OpenApiRouter, routes};

pub mod auth;
pub mod profile;

pub const B2C_TAG: &str = "B2C";

pub fn router<App>() -> OpenApiRouter<App>
where
    App: Application,
{
    OpenApiRouter::new()
        .routes(routes!(auth::sign_up::<App>))
        .routes(routes!(auth::log_in::<App>))
        .routes(routes!(profile::get_profile::<App>))
}
