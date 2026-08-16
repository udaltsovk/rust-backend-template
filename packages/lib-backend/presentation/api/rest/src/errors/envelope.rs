use axum::http::StatusCode;
use chrono::{DateTime, Utc};
use serde_json::{Map, Value};
use uuid::Uuid;

use super::validation::FieldError;

#[derive(Clone, Debug)]
pub struct ErrorData {
    pub status_code: StatusCode,
    pub error_code: &'static str,
    pub message: String,
    pub trace_id: Uuid,
    pub timestamp: DateTime<Utc>,
    pub path: String,
    pub details: Value,
    pub field_errors: Vec<FieldError>,
}

pub trait ErrorEnvelope: Send + Sync + 'static {
    fn render(&self, data: &ErrorData) -> Value;
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ErrorField {
    TraceId,
    Timestamp,
    Path,
    Details,
}

impl ErrorField {
    const fn bit(self) -> u8 {
        match self {
            Self::TraceId => 0b0001,
            Self::Timestamp => 0b0010,
            Self::Path => 0b0100,
            Self::Details => 0b1000,
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct ErrorFields(u8);

impl ErrorFields {
    pub const ALL: Self = Self(0b1111);
    pub const NONE: Self = Self(0);

    #[must_use]
    pub const fn with(self, field: ErrorField) -> Self {
        Self(self.0 | field.bit())
    }

    #[must_use]
    pub const fn without(self, field: ErrorField) -> Self {
        Self(self.0 & !field.bit())
    }

    #[must_use]
    pub const fn contains(self, field: ErrorField) -> bool {
        (self.0 & field.bit()) != 0
    }
}

impl Default for ErrorFields {
    fn default() -> Self {
        Self::ALL
    }
}

#[derive(Clone, Copy, Debug, Default)]
pub struct DefaultEnvelope {
    pub fields: ErrorFields,
}

impl DefaultEnvelope {
    #[must_use]
    pub const fn new(fields: ErrorFields) -> Self {
        Self {
            fields,
        }
    }
}

impl ErrorEnvelope for DefaultEnvelope {
    fn render(&self, data: &ErrorData) -> Value {
        let mut body = Map::new();

        body.insert(
            "code".to_owned(),
            Value::String(data.error_code.to_owned()),
        );
        body.insert("message".to_owned(), Value::String(data.message.clone()));

        if self.fields.contains(ErrorField::TraceId) {
            body.insert(
                "traceId".to_owned(),
                Value::String(data.trace_id.to_string()),
            );
        }

        if self.fields.contains(ErrorField::Timestamp) {
            body.insert(
                "timestamp".to_owned(),
                serde_json::to_value(data.timestamp).unwrap_or(Value::Null),
            );
        }

        if self.fields.contains(ErrorField::Path) {
            body.insert("path".to_owned(), Value::String(data.path.clone()));
        }

        if self.fields.contains(ErrorField::Details) && !data.details.is_null()
        {
            body.insert("details".to_owned(), data.details.clone());
        }

        if !data.field_errors.is_empty() {
            body.insert(
                "fieldErrors".to_owned(),
                serde_json::to_value(&data.field_errors).unwrap_or(Value::Null),
            );
        }

        Value::Object(body)
    }
}
