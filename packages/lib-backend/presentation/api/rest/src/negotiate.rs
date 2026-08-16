use std::sync::Arc;

use axum::{
    body::Body,
    extract::{Request, State},
    http::{
        HeaderValue,
        header::{ACCEPT, CONTENT_LENGTH, CONTENT_TYPE},
    },
    middleware::Next,
    response::Response,
};
use serde_json::Value;

use crate::errors::envelope::{DefaultEnvelope, ErrorData, ErrorEnvelope};

pub const JSON_CONTENT_TYPE: &str = "application/json";

pub trait BodyEncoder: Send + Sync + 'static {
    fn content_type(&self) -> &'static str;

    fn encode(&self, body: &Value) -> Option<Vec<u8>>;
}

#[derive(Clone)]
pub struct ResponseFormat {
    envelope: Arc<dyn ErrorEnvelope>,
    encoders: Arc<[Box<dyn BodyEncoder>]>,
}

impl ResponseFormat {
    #[must_use]
    pub fn new<E>(envelope: E) -> Self
    where
        E: ErrorEnvelope,
    {
        Self {
            envelope: Arc::new(envelope),
            encoders: Vec::new().into(),
        }
    }

    #[must_use]
    pub fn encoders(mut self, encoders: Vec<Box<dyn BodyEncoder>>) -> Self {
        self.encoders = encoders.into();
        self
    }
}

impl Default for ResponseFormat {
    fn default() -> Self {
        Self::new(DefaultEnvelope::default())
    }
}

pub async fn apply(
    State(format): State<ResponseFormat>,
    request: Request,
    next: Next,
) -> Response {
    let accept = request
        .headers()
        .get(ACCEPT)
        .and_then(|value| value.to_str().ok())
        .map(str::to_owned);

    let response = next.run(request).await;

    let Some(data) = response.extensions().get::<ErrorData>().cloned() else {
        return response;
    };

    let body = format.envelope.render(&data);

    let encoder = accept.as_ref().and_then(|accept| {
        format
            .encoders
            .iter()
            .find(|encoder| accept.contains(encoder.content_type()))
    });

    let (content_type, bytes) = match encoder {
        Some(encoder) => match encoder.encode(&body) {
            Some(bytes) => (encoder.content_type(), bytes),
            None => return response,
        },
        None => match serde_json::to_vec(&body) {
            Ok(bytes) => (JSON_CONTENT_TYPE, bytes),
            Err(_) => return response,
        },
    };

    let (mut parts, _) = response.into_parts();
    parts.headers.remove(CONTENT_LENGTH);

    if let Ok(value) = HeaderValue::from_str(content_type) {
        parts.headers.insert(CONTENT_TYPE, value);
    }

    Response::from_parts(parts, Body::from(bytes))
}
