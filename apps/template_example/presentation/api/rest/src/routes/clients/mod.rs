use lib::utoipa_axum::{router::OpenApiRouter, routes};

use crate::module::ModulesExt;

mod bulk;
mod client_id;

pub const CLIENTS_TAG: &str = "clients";

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new()
        .routes(routes!(bulk::bulk_upsert::<M>))
        .routes(routes!(client_id::find_by_id::<M>))
}
