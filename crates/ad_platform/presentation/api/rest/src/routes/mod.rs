use axum::{Json, http::StatusCode, response::IntoResponse};
use utoipa_axum::router::OpenApiRouter;

use crate::{context::response_helper::JsonErrorStruct, module::ModulesExt};

pub mod ads;
pub mod advertisers;
pub mod attachments;
pub mod campaigns;
pub mod clients;
pub mod statistics;
pub mod time;

pub fn router<M: ModulesExt>() -> OpenApiRouter<M> {
    OpenApiRouter::new()
        .nest("/clients", clients::router())
        .merge(advertisers::router())
        .nest("/ads", ads::router())
        .nest("/statistics", statistics::router())
        .nest("/time", time::router())
        .nest("/attachments", attachments::router())
}

pub async fn fallback() -> impl IntoResponse {
    let messages = vec!["specified route does not exist".to_string()];
    (
        StatusCode::NOT_FOUND,
        Json(JsonErrorStruct::new("not_found".to_string(), messages)),
    )
}
