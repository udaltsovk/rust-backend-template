use lib::utoipa_axum::{router::OpenApiRouter, routes};

use crate::Application;

pub mod profile;

pub fn router<App>() -> OpenApiRouter<App>
where
    App: Application,
{
    OpenApiRouter::new()
        .routes(routes!(profile::get_profile::<App>))
}
