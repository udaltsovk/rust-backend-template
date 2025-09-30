use utoipa_axum::router::OpenApiRouter;

use crate::{module::ModulesExt, routes::campaigns};

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new().nest("/{advertiser_id}/campaigns", campaigns::router())
}
