use axum::{Json, http::StatusCode, response::IntoResponse};

use crate::context::JsonErrorStruct;

pub async fn fallback() -> impl IntoResponse {
    let messages = vec!["the specified route does not exist".to_string()];
    (
        StatusCode::NOT_FOUND,
        Json(JsonErrorStruct::new("not_found".to_string(), messages)),
    )
}
