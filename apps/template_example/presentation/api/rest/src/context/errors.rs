use std::fmt::Debug;

use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
};
use domain::error::DomainError;
use lib::domain::validation::error::ValidationErrors;

mod client;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
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

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        use DomainError as DE;
        match error {
            DE::Validation(err) => Self::Validation(err),
        }
    }
}
