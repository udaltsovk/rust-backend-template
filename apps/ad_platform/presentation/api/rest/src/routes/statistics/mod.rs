use utoipa_axum::router::OpenApiRouter;

use crate::module::ModulesExt;

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new()
}
