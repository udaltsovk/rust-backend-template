use domain::{email::Email, user::User};
use lib::{application::usecase_result, domain::Id};

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

#[derive(thiserror::Error, Debug)]
pub enum UserUseCaseError<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    #[error("Repository error: {0}")]
    Repository(R::Error),

    #[error(transparent)]
    Service(S::Error),

    #[error("user with email `{0}` already exists")]
    EmailAlreadyUsed(Email),

    #[error("user with email `{email}` does not exist")]
    NotFoundByEmail { email: Email, from_auth: bool },

    #[error("user with id `{0}` does not exist")]
    NotFoundById(Id<User>),

    #[error("invalid password")]
    InvalidPassword,
}

usecase_result!(User);
