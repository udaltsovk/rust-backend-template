use std::{error::Error, fmt, sync::Arc};

use axum::http::StatusCode;
use domain::validation::{
    ValidationConfirmation,
    error::{ValidationError, ValidationErrors},
};
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
    #[cfg_attr(feature = "openapi", schema(value_type = String))]
    pub field: Arc<str>,

    pub issue: String,

    #[serde(skip_serializing_if = "Value::is_null")]
    pub rejected_value: Value,
}

impl fmt::Display for FieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Field: {}, Issue: {}, Rejected value: {:?}",
            self.field, self.issue, self.rejected_value
        )
    }
}

impl FieldError {
    #[must_use]
    pub fn from_validation_error(
        field: &Arc<str>,
        ValidationError {
            issue,
            rejected_value,
        }: ValidationError,
    ) -> Self {
        Self {
            field: Arc::clone(field),
            issue,
            rejected_value: serde_json::to_value(rejected_value)
                .unwrap_or_default(),
        }
    }
}

#[derive(Clone, Debug)]
#[must_use]
pub struct FieldErrors(Vec<FieldError>);

impl FieldErrors {
    pub const fn new() -> Self {
        Self(vec![])
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<FieldError> {
        self.0
    }

    pub const fn inner_mut(&mut self) -> &mut Vec<FieldError> {
        &mut self.0
    }

    #[must_use]
    pub fn inner(&self) -> &[FieldError] {
        &self.0
    }

    pub fn extend(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "clippy doesn't know that both P and M may be &str"
    )]
    pub fn push<P, M, V>(&mut self, path: P, issue: M, rejected_value: V)
    where
        P: Into<Arc<str>>,
        M: ToString,
        V: Serialize,
    {
        let error = FieldError {
            field: path.into(),
            issue: issue.to_string(),
            rejected_value: serde_json::to_value(rejected_value)
                .unwrap_or_default(),
        };
        self.0.push(error);
    }

    pub fn into_result<T, F>(self, ok_fn: F) -> Result<T, Self>
    where
        F: FnOnce(ValidationConfirmation) -> T,
    {
        self.0
            .is_empty()
            .then(|| {
                ValidationErrors::new().into_result(ok_fn).expect(
                    "we've checked for errors here so it should be safe",
                )
            })
            .ok_or(self)
    }
}

impl Default for FieldErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for FieldErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errors = self
            .0
            .iter()
            .map(FieldError::to_string)
            .collect::<Vec<_>>()
            .join("\n");

        write!(f, "Field errors: [\n{errors}\n]")
    }
}

impl Error for FieldErrors {}

impl FieldErrors {
    pub fn from_validation_errors(
        field: &Arc<str>,
        errors: ValidationErrors,
    ) -> Self {
        errors
            .into_inner()
            .into_iter()
            .map(|err| FieldError::from_validation_error(field, err))
            .collect::<Vec<_>>()
            .pipe(Self)
    }
}

impl From<Vec<Self>> for FieldErrors {
    fn from(errors: Vec<Self>) -> Self {
        errors
            .into_iter()
            .fold(Self::default(), |mut accumulator, error| {
                accumulator.extend(Self(error.0));
                accumulator
            })
    }
}

impl From<FieldErrors> for Vec<FieldError> {
    fn from(errors: FieldErrors) -> Self {
        errors.into_inner()
    }
}

impl FromIterator<Self> for FieldErrors {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl FromIterator<FieldError> for FieldErrors {
    fn from_iter<T: IntoIterator<Item = FieldError>>(iter: T) -> Self {
        Self(iter.into_iter().collect())
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
macro_rules! fields_error_response_openapi {
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
