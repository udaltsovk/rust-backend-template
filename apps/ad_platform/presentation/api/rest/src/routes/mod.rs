use utoipa::OpenApi as _;
use utoipa_axum::router::OpenApiRouter;

use crate::{context::openapi::ApiDoc, module::ModulesExt};

pub mod ads;
pub mod advertisers;
pub mod attachments;
pub mod campaigns;
pub mod clients;
pub mod statistics;
pub mod time;

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::with_openapi(ApiDoc::openapi())
        .nest("/clients", clients::router())
        .merge(advertisers::router())
        .nest("/ads", ads::router())
        .nest("/statistics", statistics::router())
        .nest("/time", time::router())
        .nest("/attachments", attachments::router())
}
