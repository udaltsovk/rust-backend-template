use axum::{http::StatusCode, response::IntoResponse};

use crate::context::JsonErrorStruct;

pub async fn fallback() -> impl IntoResponse {
    let messages = vec!["the specified route does not exist".to_string()];
    JsonErrorStruct::new(
        StatusCode::NOT_FOUND,
        "not_found".to_string(),
        messages,
    )
}
