use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::user::error::UserUseCaseError,
};
use axum::http::StatusCode;
use lib::presentation::api::rest::context::InternalErrorStringExt as _;

use crate::context::errors::AppError;

impl<R, S> From<UserUseCaseError<R, S>> for AppError
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn from(error: UserUseCaseError<R, S>) -> Self {
        let (status_code, error_code) = {
            use StatusCode as C;
            use UserUseCaseError as E;
            match error {
                E::Repository(_) | E::Service(_) => {
                    (C::INTERNAL_SERVER_ERROR, "internal_server_error")
                },
                E::EmailAlreadyUsed(_) => (C::CONFLICT, "email_already_used"),
            }
        };

        Self::UseCase {
            status_code,
            error_code,
            error: error.to_internal_error_string(),
        }
    }
}
