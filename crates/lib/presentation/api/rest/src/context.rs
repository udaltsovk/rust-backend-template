use std::fmt::Display;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse as _, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct JsonErrorStruct {
    error_code: String,
    errors: Vec<String>,
}

impl JsonErrorStruct {
    pub fn new(error_code: impl Display, errors: Vec<impl Display>) -> Self {
        Self {
            error_code: error_code.to_string(),
            errors: errors.into_iter().map(|e| e.to_string()).collect(),
        }
    }

    pub fn as_response(&self, status_code: StatusCode) -> Response {
        (status_code, Json(self)).into_response()
    }
}
