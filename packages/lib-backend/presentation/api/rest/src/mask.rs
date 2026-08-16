use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::Response,
};
use chrono::Utc;
use result_like::BoolLike;
use serde_json::Value;
use uuid::Uuid;

use crate::errors::envelope::ErrorData;

#[derive(BoolLike, PartialEq, Eq, Clone, Copy, Debug)]
pub enum ServerErrorMasking {
    Enabled,
    Disabled,
}

pub async fn server_errors(request: Request, next: Next) -> Response {
    mask(next.run(request).await)
}

pub async fn server_errors_if(
    State(masking): State<ServerErrorMasking>,
    request: Request,
    next: Next,
) -> Response {
    let response = next.run(request).await;

    if masking.to_bool() {
        mask(response)
    } else {
        response
    }
}

fn mask(mut response: Response) -> Response {
    if !response.status().is_server_error() {
        return response;
    }

    let original = response.extensions().get::<ErrorData>().cloned();

    let masked = ErrorData {
        status_code: StatusCode::INTERNAL_SERVER_ERROR,
        error_code: "INTERNAL_ERROR",
        message: "an internal error occurred".to_owned(),
        trace_id: original
            .as_ref()
            .map_or_else(Uuid::nil, |data| data.trace_id),
        timestamp: original
            .as_ref()
            .map_or_else(Utc::now, |data| data.timestamp),
        path: original
            .as_ref()
            .map_or_else(String::new, |data| data.path.clone()),
        details: Value::Null,
        field_errors: Vec::new(),
    };

    *response.status_mut() = StatusCode::INTERNAL_SERVER_ERROR;
    response.extensions_mut().insert(masked);

    response
}
