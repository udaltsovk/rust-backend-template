use std::fmt::Display;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

/// Error response structure
#[derive(Serialize, ToSchema, Debug)]
pub struct JsonErrorStruct {
    /// Response status code
    #[serde(skip)]
    pub(crate) status_code: StatusCode,

    /// Response error code
    pub(crate) error_code: String,

    /// Response error list
    pub(crate) errors: Vec<String>,
}

impl JsonErrorStruct {
    pub fn new(
        status_code: impl Into<StatusCode>,
        error_code: impl Display,
        errors: Vec<impl Display>,
    ) -> Self {
        Self {
            status_code: status_code.into(),
            error_code: error_code.to_string(),
            errors: errors.into_iter().map(|e| e.to_string()).collect(),
        }
    }
}

impl IntoResponse for JsonErrorStruct {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}
