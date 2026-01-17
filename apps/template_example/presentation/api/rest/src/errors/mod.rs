use std::fmt::Debug;

use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
};
use domain::error::DomainError;
use lib::{
    domain::validation::error::ValidationErrors,
    presentation::api::rest::context::{
        InternalErrorStringExt as _, JsonErrorStruct,
    },
};
use tracing::{error, warn};

pub use crate::errors::{auth::AuthError, bad_request::BadRequestResponse};

mod auth;
mod bad_request;
mod usecase;

#[derive(thiserror::Error, Debug)]
pub enum ApiError {
    #[error(transparent)]
    Validation(#[from] ValidationErrors),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    ApiPathRejection(#[from] PathRejection),

    #[error("{0}")]
    UnknownApiVerRejection(String),

    #[error("{error}")]
    UseCase {
        status_code: StatusCode,
        error_code: &'static str,
        error: String,
    },
}

impl ApiError {
    pub fn internal_server_error<T>(
        error: T,
    ) -> (StatusCode, &'static str, String)
    where
        T: ToString,
    {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            "internal_server_error",
            error.to_internal_error_string(
                "Something went wrong on our side...",
            ),
        )
    }
}

impl From<DomainError> for ApiError {
    fn from(error: DomainError) -> Self {
        use DomainError as DE;
        match error {
            DE::Validation(err) => Self::Validation(err),
        }
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
                BadRequestResponse::from(validation_errors).into()
            },
            Self::JsonRejection(rejection) => {
                BadRequestResponse::from(rejection).into()
            },
            Self::ApiPathRejection(rejection) => {
                BadRequestResponse::from(rejection).into()
            },
            Self::UnknownApiVerRejection(version) => JsonErrorStruct::new(
                StatusCode::NOT_FOUND,
                "unknown_api_version",
                vec![format!("Unknown api version ({version}).")],
            ),
            Self::UseCase {
                status_code,
                error_code,
                error,
            } => JsonErrorStruct::new(status_code, error_code, vec![error]),
        }
        .into_response()
    }
}
