use axum::{
    Json,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use serde::Serialize;
use serde_json::Value;
use tracing::Span;
use tracing_subscriber::registry::LookupSpan as _;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;
use uuid::Uuid;

use self::{
    envelope::{DefaultEnvelope, ErrorData, ErrorEnvelope as _},
    generic::GenericJsonError,
    validation::ValidationJsonError,
};

pub mod envelope;
pub mod generic;
pub mod validation;

#[derive(Debug)]
pub struct RequestMeta {
    pub http_route: Uri,
    pub request_id: Option<Uuid>,
}

#[derive(Serialize, Debug)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(untagged, rename_all = "camelCase")]
pub enum JsonError {
    Generic(GenericJsonError),
    Validation(ValidationJsonError),
}

#[derive(Serialize, Debug)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct JsonErrorStruct {
    #[serde(skip)]
    pub status_code: StatusCode,

    #[serde(rename = "code")]
    pub error_code: &'static str,

    pub message: String,

    pub trace_id: Uuid,

    pub timestamp: DateTime<Utc>,

    #[cfg_attr(feature = "openapi", schema(format = Uri))]
    pub path: String,
}

impl JsonErrorStruct {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "we may pass &str here and then &M won't work"
    )]
    pub fn new<S, M>(
        status_code: S,
        error_code: &'static str,
        message: M,
    ) -> Self
    where
        S: Into<StatusCode>,
        M: ToString,
    {
        let mut http_route_option: Option<Uri> = None;
        let mut request_id_option: Option<Uuid> = None;

        Span::current().with_subscriber(|(id, subscriber)| {
            if let Some(registry) =
                subscriber.downcast_ref::<tracing_subscriber::Registry>()
                && let Some(span_ref) = registry.span(id)
                && let Some(meta) = span_ref.extensions().get::<RequestMeta>()
            {
                http_route_option = Some(meta.http_route.clone());
                request_id_option = meta.request_id;
            }
        });

        let http_route =
            http_route_option.unwrap_or_else(|| Uri::from_static("unknown"));
        let request_id = request_id_option.unwrap_or(Uuid::nil());

        Self {
            status_code: status_code.into(),
            error_code,
            message: message.to_string(),
            trace_id: request_id,
            timestamp: Utc::now(),
            path: http_route.to_string(),
        }
    }
}

impl JsonError {
    #[must_use]
    pub const fn inner_struct(&self) -> &JsonErrorStruct {
        match self {
            Self::Generic(GenericJsonError {
                error, ..
            })
            | Self::Validation(ValidationJsonError {
                error, ..
            }) => error,
        }
    }
}

impl IntoResponse for JsonError {
    fn into_response(self) -> Response {
        static NO_DETAILS: Value = Value::Null;

        let (details, field_errors) = match &self {
            Self::Generic(generic) => (&generic.details, [].as_slice()),
            Self::Validation(validation) => {
                (&NO_DETAILS, validation.field_errors.as_slice())
            },
        };

        let inner = self.inner_struct();
        let status_code = inner.status_code;

        let data = ErrorData {
            status_code,
            error_code: inner.error_code,
            message: inner.message.clone(),
            trace_id: inner.trace_id,
            timestamp: inner.timestamp,
            path: inner.path.clone(),
            details: details.clone(),
            field_errors: field_errors.to_vec(),
        };

        let body = DefaultEnvelope::default().render(&data);

        let mut response = (status_code, Json(&body)).into_response();

        response.extensions_mut().insert(data);

        response
    }
}

pub trait InternalErrorStringExt: ToString + Sized {
    fn to_internal_error_string(self, public: &'static str) -> String {
        self.to_internal_error_string_with_debug(cfg!(debug_assertions), public)
    }

    fn to_internal_error_string_with_debug(
        self,
        is_debug: bool,
        public: &'static str,
    ) -> String {
        if is_debug {
            self.to_string()
        } else {
            public.to_string()
        }
    }
}

impl<T> InternalErrorStringExt for T where T: ToString + Sized {}
