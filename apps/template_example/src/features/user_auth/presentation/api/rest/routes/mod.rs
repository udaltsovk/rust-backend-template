use lib::utoipa_axum::{router::OpenApiRouter, routes};

use crate::Application;

pub mod sign_in;
pub mod sign_up;

pub fn router<App>() -> OpenApiRouter<App>
where
    App: Application,
{
    OpenApiRouter::new()
        .routes(routes!(sign_up::sign_up::<App>))
        .routes(routes!(sign_in::sign_in::<App>))
}
