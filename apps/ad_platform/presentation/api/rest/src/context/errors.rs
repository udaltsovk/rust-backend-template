use std::fmt::Debug;

use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::client::error::ClientUseCaseError,
};
use axum::{
    extract::rejection::{JsonRejection, PathRejection},
    http::StatusCode,
};
use domain::error::DomainError;
use lib::domain::validation::error::ValidationErrors;

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

    #[error("{message}")]
    UseCase {
        status_code: StatusCode,
        error_code: &'static str,
        message: String,
    },
}

impl<R, S> From<ClientUseCaseError<R, S>> for AppError
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn from(error: ClientUseCaseError<R, S>) -> Self {
        let (status_code, error_code) = {
            use ClientUseCaseError as E;
            use StatusCode as C;
            match error {
                E::Repository(_) | E::Service(_) => {
                    (C::INTERNAL_SERVER_ERROR, "internal_server_error")
                },
            }
        };

        AppError::UseCase {
            status_code,
            error_code,
            message: error.to_string(),
        }
    }
}

impl From<DomainError> for AppError {
    fn from(error: DomainError) -> Self {
        use DomainError as DE;
        match error {
            DE::Validation(err) => Self::Validation(err),
        }
    }
}
