use axum::{
    extract::{FromRequest, rejection::JsonRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

use crate::context::JsonErrorStruct;

#[derive(FromRequest)]
#[from_request(via(axum::Json), rejection(JsonErrorStruct))]
pub struct Json<T>(pub T);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        let Self(value) = self;
        axum::Json(value).into_response()
    }
}

impl From<JsonRejection> for JsonErrorStruct {
    fn from(rejection: JsonRejection) -> Self {
        Self {
            status_code: StatusCode::BAD_REQUEST,
            error_code: "invalid_json".to_string(),
            errors: vec![rejection.body_text()],
        }
    }
}
