use lib::utoipa_axum::router::OpenApiRouter;

use crate::module::ModulesExt;

#[must_use]
pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new()
}
