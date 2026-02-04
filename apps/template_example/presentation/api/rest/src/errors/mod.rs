use std::fmt::Debug;

use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib::presentation::api::rest::errors::{
    InternalErrorStringExt as _, JsonError, validation::FieldErrors,
};
use serde_json::Value;
use tracing::{error, warn};

pub use crate::errors::{
    auth::AuthError, bad_request::BadRequestResponse,
    validation::ValidationFailedResponse,
};

mod auth;
mod bad_request;
mod usecase;
mod validation;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Validation(#[from] FieldErrors),
    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    ApiPathRejection(#[from] PathRejection),

    #[error("{0}")]
    UnknownApiVerRejection(String),

    #[error("{message}")]
    UseCase {
        status_code: StatusCode,
        error_code: &'static str,
        message: String,
        details: Value,
    },
}

impl ApiError {
    pub fn internal_server_error<T>(
        error: T,
    ) -> (StatusCode, &'static str, String, Value)
    where
        T: ToString,
    {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_server_error",
            error.to_internal_error_string(
                "Something went wrong on our side...",
            ),
            Value::Null,
        )
    }
}

impl ApiError {
    fn log(&self) {
        match self {
            Self::Validation(_)
            | Self::JsonRejection(_)
            | Self::ApiPathRejection(_)
            | Self::UnknownApiVerRejection(_) => warn!("{self:?}"),

            Self::UseCase {
                status_code, ..
            } => self.log_usecase(*status_code),
        }
    }

    fn log_usecase(&self, status_code: StatusCode) {
        match status_code {
            c if c.is_server_error() => error!("{self:?}"),
            c if c.is_client_error() => warn!("{self:?}"),
            _ => (),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        self.log();
        match self {
            Self::Validation(validation_errors) => {
                ValidationFailedResponse::from(validation_errors).into()
            },
            Self::JsonRejection(rejection) => {
                BadRequestResponse::from(rejection).into()
            },
            Self::ApiPathRejection(rejection) => {
                BadRequestResponse::from(rejection).into()
            },
            Self::UnknownApiVerRejection(version) => JsonError::new(
                StatusCode::NOT_FOUND,
                "unknown_api_version",
                format!("Unknown api version ({version})."),
            ),
            Self::UseCase {
                status_code,
                error_code,
                message: error,
                details,
            } => {
                JsonError::with_details(status_code, error_code, error, details)
                    .expect("details from value should serialize successfully")
            },
        }
        .into_response()
    }
}
