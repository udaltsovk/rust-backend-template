use lib::utoipa_axum::router::OpenApiRouter;
use utoipa::OpenApi as _;

use crate::{ApiDoc, ModulesExt};

pub mod user;

#[must_use]
pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::with_openapi(ApiDoc::openapi()).nest("/user", user::router())
}
