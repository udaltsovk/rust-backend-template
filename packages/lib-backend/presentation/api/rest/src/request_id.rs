use std::str::FromStr as _;

use axum::{
    extract::{Request, State},
    http::{HeaderName, HeaderValue, StatusCode},
    middleware::Next,
    response::{IntoResponse as _, Response},
};
use uuid::Uuid;

use crate::errors::JsonError;

pub const HEADER: &str = "x-request-id";

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub enum RequestIdPolicy {
    Reject,
    Require,
    #[default]
    Generate,
}

pub async fn enforce(
    State(policy): State<RequestIdPolicy>,
    mut request: Request,
    next: Next,
) -> Response {
    let header = HeaderName::from_static(HEADER);

    if policy == RequestIdPolicy::Generate {
        match HeaderValue::from_str(&Uuid::now_v7().to_string()) {
            Ok(value) => {
                request.headers_mut().insert(&header, value);
            },
            Err(_) => {
                request.headers_mut().remove(&header);
            },
        }

        return next.run(request).await;
    }

    let present = request.headers().get(&header);

    let valid = present.is_some_and(|value| {
        value
            .to_str()
            .ok()
            .and_then(|value| Uuid::from_str(value).ok())
            .is_some()
    });

    let rejected = match policy {
        RequestIdPolicy::Require => !valid,
        RequestIdPolicy::Reject => present.is_some() && !valid,
        RequestIdPolicy::Generate => false,
    };

    if rejected {
        return JsonError::new(
            StatusCode::BAD_REQUEST,
            "INVALID_REQUEST_ID",
            "`x-request-id` must be a valid UUID",
        )
        .into_response();
    }

    next.run(request).await
}
