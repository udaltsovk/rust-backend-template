use axum::http::StatusCode;
use domain::validation::error::{ValidationError, ValidationErrors};
use serde::Serialize;
use serde_json::Value;
use tap::Pipe as _;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::errors::{JsonError, JsonErrorStruct};

#[derive(Serialize, Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
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
            rejected_value: serde_json::to_value(rejected_value)
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone)]
#[cfg_attr(debug_assertions, derive(Debug))]
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

#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct ValidationJsonError {
    #[serde(flatten)]
    pub error: JsonErrorStruct,

    pub field_errors: Vec<FieldError>,
}

impl JsonError {
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
}

#[cfg(feature = "openapi")]
#[macro_export]
macro_rules! validation_error_response_openapi {
    (
        $(#[$meta:meta])*
        name = $name: ident,
        error_code = $error_code: literal,
        status_code = $status_code: ident $(,)*
    ) => {
        $crate::validation_error_response!(
            $(#[$meta])*
            #[derive(utoipa::ToSchema, utoipa::IntoResponses)]
            #[response(status = $status_code)]
            name = $name,
            error_code = $error_code,
            status_code = $status_code
        );
    };
}

#[macro_export]
macro_rules! validation_error_response {
    (
        $(#[$meta:meta])*
        name = $name: ident,
        error_code = $error_code: literal,
        status_code = $status_code: ident $(,)*
    ) => {
        $(#[$meta])*
        #[derive(serde::Serialize)]
        pub struct $name($crate::errors::validation::ValidationJsonError);

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
                $crate::errors::validation::FieldErrors: From<E>
            {
                Self($crate::errors::validation::ValidationJsonError {
                    error: $crate::errors::JsonErrorStruct::new(Self::STATUS_CODE, Self::ERROR_CODE, message),
                    field_errors: $crate::errors::validation::FieldErrors::from(errors).into()
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
