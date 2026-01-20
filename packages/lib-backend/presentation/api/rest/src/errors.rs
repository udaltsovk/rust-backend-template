use axum::{
    Json,
    http::{StatusCode, Uri},
    response::{IntoResponse, Response},
};
use chrono::{DateTime, Utc};
use domain::validation::error::{ValidationError, ValidationErrors};
use serde::Serialize;
use serde_json::Value;
use tap::Pipe as _;
use tracing::Span;
use tracing_subscriber::registry::LookupSpan as _;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug)]
pub struct RequestMeta {
    pub http_route: Option<Uri>,
    pub request_id: Option<Uuid>,
}

#[derive(Serialize, ToSchema, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FieldError {
    pub field: String,

    pub issue: String,

    #[serde(skip_serializing_if = "Value::is_null")]
    pub rejected_value: Value,
}

impl From<ValidationError> for FieldError {
    fn from(
        ValidationError {
            path,
            issue,
            rejected_value,
        }: ValidationError,
    ) -> Self {
        Self {
            field: path,
            issue,
            rejected_value,
        }
    }
}

#[derive(Clone, Debug)]
pub struct FieldErrors(Vec<FieldError>);

impl FieldErrors {
    #[must_use]
    pub const fn new() -> Self {
        Self(vec![])
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<FieldError> {
        self.0
    }

    pub fn extend(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "clippy doesn't know that both P and M might be &str"
    )]
    pub fn push<P, M, V>(&mut self, path: P, issue: M, rejected_value: V)
    where
        P: ToString,
        M: ToString,
        V: Serialize,
    {
        let error = FieldError {
            field: path.to_string(),
            issue: issue.to_string(),
            rejected_value: serde_json::to_value(rejected_value)
                .unwrap_or_default(),
        };
        self.0.push(error);
    }
}

impl Default for FieldErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl From<ValidationErrors> for FieldErrors {
    fn from(errors: ValidationErrors) -> Self {
        errors
            .into_inner()
            .into_iter()
            .map(FieldError::from)
            .collect::<Vec<_>>()
            .pipe(Self)
    }
}

impl From<FieldErrors> for Vec<FieldError> {
    fn from(errors: FieldErrors) -> Self {
        errors.into_inner()
    }
}

#[derive(Serialize, ToSchema, Debug)]
#[serde(untagged, rename_all = "camelCase")]
pub enum JsonError {
    Generic(GenericJsonError),
    Validation(ValidationJsonError),
}

#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct GenericJsonError {
    #[serde(flatten)]
    pub error: JsonErrorStruct,

    #[serde(skip_serializing_if = "Value::is_null")]
    pub context: Value,
}

#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ValidationJsonError {
    #[serde(flatten)]
    pub error: JsonErrorStruct,

    pub field_errors: Vec<FieldError>,
}

#[derive(Serialize, ToSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JsonErrorStruct {
    #[serde(skip)]
    pub status_code: StatusCode,

    #[serde(rename = "code")]
    pub error_code: &'static str,

    pub message: String,

    pub trace_id: Uuid,

    pub timestamp: DateTime<Utc>,

    #[schema(format = Uri)]
    pub path: String,
}

impl JsonErrorStruct {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "we might pass &str here and then &M won't work"
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
                && let Some(http_route) = &meta.http_route
                && let Some(request_id) = &meta.request_id
            {
                http_route_option = Some(http_route.clone());
                request_id_option = Some(*request_id);
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
    pub fn with_context<S, M, C>(
        status_code: S,
        error_code: &'static str,
        message: M,
        context: C,
    ) -> Result<Self, serde_json::Error>
    where
        S: Into<StatusCode>,
        M: ToString,
        C: Serialize,
    {
        Self::Generic(GenericJsonError {
            error: JsonErrorStruct::new(status_code, error_code, message),
            context: serde_json::to_value(context)?,
        })
        .pipe(Ok)
    }

    pub fn new<S, M>(
        status_code: S,
        error_code: &'static str,
        message: M,
    ) -> Self
    where
        S: Into<StatusCode>,
        M: ToString,
    {
        Self::Generic(GenericJsonError {
            error: JsonErrorStruct::new(status_code, error_code, message),
            context: Value::Null,
        })
    }

    pub fn validation<S, M, E>(
        status_code: S,
        error_code: &'static str,
        message: M,
        errors: E,
    ) -> Self
    where
        S: Into<StatusCode>,
        M: ToString,
        E: Into<Vec<FieldError>>,
    {
        Self::Validation(ValidationJsonError {
            error: JsonErrorStruct::new(status_code, error_code, message),
            field_errors: errors.into(),
        })
    }

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
        (self.inner_struct().status_code, Json(self)).into_response()
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

#[macro_export]
macro_rules! generic_error_response {
    (
        $(#[$meta:meta])*
        name = $name: ident,
        error_code = $error_code: literal,
        status_code = $status_code: ident $(,)+
    ) => {
        $(#[$meta])*
        #[derive(serde::Serialize, utoipa::ToSchema, utoipa::IntoResponses)]
        #[response(status = $status_code)]
        pub struct $name($crate::errors::GenericJsonError);

        impl $name {
            const ERROR_CODE: &str = $error_code;
            const STATUS_CODE: axum::http::StatusCode = axum::http::StatusCode::$status_code;

            #[must_use]
            pub const fn error_code() -> &'static str {
                Self::ERROR_CODE
            }

            #[must_use]
            pub fn with_context<M, C>(message: M, context: C) -> Result<Self, $crate::serde_json::Error>
            where
                M: ToString,
                C: serde::Serialize,
            {
                Ok(Self($crate::errors::GenericJsonError {
                    error: $crate::errors::JsonErrorStruct::new(Self::STATUS_CODE, Self::ERROR_CODE, message),
                    context: $crate::serde_json::to_value(context)?
                }))
            }

            #[must_use]
            pub fn new<M>(message: M) -> Self
            where
                M: ToString,
            {
                Self($crate::errors::GenericJsonError {
                    error: $crate::errors::JsonErrorStruct::new(Self::STATUS_CODE, Self::ERROR_CODE, message),
                    context: $crate::serde_json::Value::Null
                })
            }
        }

        impl From<$name> for $crate::errors::JsonError {
            fn from(error: $name) -> Self {
                Self::with_context(
                    $name::STATUS_CODE,
                    $name::ERROR_CODE,
                    error.0.error.message,
                    error.0.context
                )
                .expect("context is already a value so it should be ok")
            }
        }
    };
}

#[macro_export]
macro_rules! validation_error_response {
    (
        $(#[$meta:meta])*
        name = $name: ident,
        error_code = $error_code: literal,
        status_code = $status_code: ident $(,)+
    ) => {
        $(#[$meta])*
        #[derive(utoipa::ToSchema, utoipa::IntoResponses)]
        #[response(status = $status_code)]
        pub struct $name($crate::errors::ValidationJsonError);

        impl $name {
            const ERROR_CODE: &str = $error_code;
            const STATUS_CODE: axum::http::StatusCode = axum::http::StatusCode::$status_code;

            #[must_use]
            pub const fn error_code() -> &'static str {
                Self::ERROR_CODE
            }

            #[must_use]
            pub fn new<M, E>(message: M, errors: E) -> Self
            where
                M: ToString,
                $crate::errors::FieldErrors: From<E>
            {
                Self($crate::errors::ValidationJsonError {
                    error: $crate::errors::JsonErrorStruct::new(Self::STATUS_CODE, Self::ERROR_CODE, message),
                    field_errors: $crate::errors::FieldErrors::from(errors).into()
                })
            }
        }

        impl From<$name> for $crate::errors::JsonError {
            fn from(error: $name) -> Self {
                Self::validation(
                    $name::STATUS_CODE,
                    $name::ERROR_CODE,
                    error.0.error.message,
                    error.0.field_errors
                )
            }
        }
    };
}
