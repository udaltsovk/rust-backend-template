use axum::http::StatusCode;
use serde::Serialize;
use serde_json::Value;
use tap::Pipe as _;
#[cfg(feature = "openapi")]
use utoipa::ToSchema;

use crate::errors::{JsonError, JsonErrorStruct};

#[derive(Serialize)]
#[cfg_attr(debug_assertions, derive(Debug))]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
#[serde(rename_all = "camelCase")]
pub struct GenericJsonError {
    #[serde(flatten)]
    pub error: JsonErrorStruct,

    #[serde(skip_serializing_if = "Value::is_null")]
    pub details: Value,
}

impl JsonError {
    pub fn with_details<S, M, D>(
        status_code: S,
        error_code: &'static str,
        message: M,
        details: D,
    ) -> Result<Self, serde_json::Error>
    where
        S: Into<StatusCode>,
        M: ToString,
        D: Serialize,
    {
        Self::Generic(GenericJsonError {
            error: JsonErrorStruct::new(status_code, error_code, message),
            details: serde_json::to_value(details)?,
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
            details: Value::Null,
        })
    }
}

#[cfg(feature = "openapi")]
#[macro_export]
macro_rules! generic_error_response_openapi {
    (
        $(#[$meta:meta])*
        name = $name: ident,
        error_code = $error_code: literal,
        status_code = $status_code: ident $(,)*
    ) => {
        $crate::generic_error_response!(
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
macro_rules! generic_error_response {
    (
        $(#[$meta:meta])*
        name = $name: ident,
        error_code = $error_code: literal,
        status_code = $status_code: ident $(,)*
    ) => {
        $(#[$meta])*
        #[derive(serde::Serialize)]
        pub struct $name($crate::errors::generic::GenericJsonError);

        impl $name {
            const ERROR_CODE: &str = $error_code;
            const STATUS_CODE: axum::http::StatusCode = axum::http::StatusCode::$status_code;

            #[must_use]
            pub const fn error_code() -> &'static str {
                Self::ERROR_CODE
            }

            #[must_use]
            pub fn with_details<M, D>(message: M, details: D) -> Result<Self, $crate::serde_json::Error>
            where
                M: ToString,
                D: serde::Serialize,
            {
                Ok(Self($crate::errors::generic::GenericJsonError {
                    error: $crate::errors::JsonErrorStruct::new(Self::STATUS_CODE, Self::ERROR_CODE, message),
                    details: $crate::serde_json::to_value(details)?
                }))
            }

            #[must_use]
            pub fn new<M>(message: M) -> Self
            where
                M: ToString,
            {
                Self($crate::errors::generic::GenericJsonError {
                    error: $crate::errors::JsonErrorStruct::new(Self::STATUS_CODE, Self::ERROR_CODE, message),
                    details: $crate::serde_json::Value::Null
                })
            }
        }

        impl From<$name> for $crate::errors::JsonError {
            fn from(error: $name) -> Self {
                Self::with_details(
                    $name::STATUS_CODE,
                    $name::ERROR_CODE,
                    error.0.error.message,
                    error.0.details
                )
                .expect("details is already a value so it should be ok")
            }
        }
    };
}
