use std::fmt::Debug;

use axum::extract::rejection::{JsonRejection, PathRejection};
use kernel::{
    application::{
        repository::RepositoriesModuleExt, service::ServicesModuleExt,
        usecase::client::error::ClientUseCaseError,
    },
    domain::error::DomainError,
};
use lib::kernel::domain::validation::error::ValidationErrors;

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

    #[error("{0}")]
    UseCase(String),
}
impl<R, S> From<ClientUseCaseError<R, S>> for AppError
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn from(e: ClientUseCaseError<R, S>) -> Self {
        AppError::UseCase(format!("{e:?}"))
    }
}
impl From<DomainError> for AppError {
    fn from(err: DomainError) -> Self {
        use DomainError as DE;
        match err {
            DE::Validation(err) => Self::Validation(err),
        }
    }
}
