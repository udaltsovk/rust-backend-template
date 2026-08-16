use lib::{
    presentation::api::rest::routes::Application,
    utoipa_axum::{router::OpenApiRouter, routes},
};

use crate::features::{
    user::application::usecase::{AuthorizeUserUsecase, CreateUserUsecase},
    user_auth::UserAuthFeature,
};

pub mod sign_in;
pub mod sign_up;

pub fn router<App>() -> OpenApiRouter<App>
where
    App: Application
        + UserAuthFeature
        + CreateUserUsecase
        + AuthorizeUserUsecase,
{
    OpenApiRouter::new()
        .routes(routes!(sign_up::sign_up::<App>))
        .routes(routes!(sign_in::sign_in::<App>))
}
