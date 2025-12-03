use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::client::error::ClientUseCaseError,
};
use axum::http::StatusCode;
use lib::presentation::api::rest::context::InternalErrorStringExt as _;

use crate::context::errors::AppError;

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

        Self::UseCase {
            status_code,
            error_code,
            error: error.to_internal_error_string(),
        }
    }
}
