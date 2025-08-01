use std::fmt::Debug;

use axum::extract::rejection::{JsonRejection, PathRejection};
use kernel::application::usecase::error::UseCaseError;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Validation(#[from] garde::Report),

    #[error(transparent)]
    JsonRejection(#[from] JsonRejection),

    #[error(transparent)]
    ApiPathRejection(#[from] PathRejection),

    #[error("{0}")]
    UnknownApiVerRejection(String),

    #[error("{0}")]
    UseCase(String),
}
impl<E: Debug> From<UseCaseError<E>> for AppError {
    fn from(e: UseCaseError<E>) -> Self {
        AppError::UseCase(format!("{e:?}"))
    }
}
