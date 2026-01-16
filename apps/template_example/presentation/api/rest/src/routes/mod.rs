use lib::utoipa_axum::router::OpenApiRouter;
use utoipa::OpenApi as _;

use crate::{context::openapi::ApiDoc, module::ModulesExt};

mod user;

#[must_use]
pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/user", user::router())
}
