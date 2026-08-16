use lib::{
    presentation::api::rest::routes::Application,
    utoipa_axum::{router::OpenApiRouter, routes},
};

use crate::features::{
    user::UserFeature,
    user_auth::application::usecase::session::GetSessionFromTokenUsecase,
};

pub mod profile;

pub fn router<App>() -> OpenApiRouter<App>
where
    App: Application + UserFeature + GetSessionFromTokenUsecase,
{
    OpenApiRouter::new().routes(routes!(profile::get_profile::<App>))
}
