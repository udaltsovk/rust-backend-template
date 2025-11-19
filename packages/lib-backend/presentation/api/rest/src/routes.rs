use axum::{http::StatusCode, response::IntoResponse};

use crate::context::JsonErrorStruct;

pub async fn fallback_404() -> impl IntoResponse {
    JsonErrorStruct::new(
        StatusCode::NOT_FOUND,
        "not_found".to_string(),
        vec!["the specified route does not exist".to_string()],
    )
}

pub async fn fallback_405() -> impl IntoResponse {
    JsonErrorStruct::new(
        StatusCode::METHOD_NOT_ALLOWED,
        "method_not_allowed".to_string(),
        vec!["the specified route does not support this method".to_string()],
    )
}
