use lib::utoipa_axum::router::OpenApiRouter;
use utoipa::OpenApi as _;

use super::ApiDoc;
use crate::{
    Application,
    features::{
        user::presentation::api::rest::user_router,
        user_auth::presentation::api::rest::user_auth_router,
    },
};

#[must_use]
pub fn router<App>() -> OpenApiRouter<App>
where
    App: Application,
{
    OpenApiRouter::with_openapi(ApiDoc::openapi()).nest(
        "/user",
        user_router().nest("/auth", user_auth_router()),
    )
}
