use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::user::error::UserUseCaseError,
};
use axum::http::StatusCode;

use crate::ApiError;

impl<R, S> From<UserUseCaseError<R, S>> for ApiError
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    fn from(error: UserUseCaseError<R, S>) -> Self {
        let (status_code, error_code, error) = {
            use StatusCode as C;
            use UserUseCaseError as E;
            match error {
                E::Repository(_) | E::Service(_) => {
                    Self::internal_server_error(error)
                },

                E::EmailAlreadyUsed(_) => {
                    (C::CONFLICT, "email_already_used", error.to_string())
                },

                E::InvalidPassword => Self::invalid_credentials(error),

                E::NotFoundByEmail {
                    from_auth, ..
                } if from_auth => Self::invalid_credentials(error),

                E::NotFoundByEmail {
                    ..
                }
                | E::NotFoundById(_) => {
                    (C::NOT_FOUND, "user_not_found", error.to_string())
                },
            }
        };

        Self::UseCase {
            status_code,
            error_code,
            error,
        }
    }
}
