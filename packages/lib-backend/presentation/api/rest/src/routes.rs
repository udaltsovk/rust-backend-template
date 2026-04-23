use axum::{http::StatusCode, response::IntoResponse};

use super::errors::JsonError;

pub async fn fallback_404() -> impl IntoResponse {
    JsonError::new(
        StatusCode::NOT_FOUND,
        "NOT_FOUND",
        "the specified route does not exist",
    )
}

pub async fn fallback_405() -> impl IntoResponse {
    JsonError::new(
        StatusCode::METHOD_NOT_ALLOWED,
        "METHOD_NOT_ALLOWED",
        "the specified route does not support this method",
    )
}
