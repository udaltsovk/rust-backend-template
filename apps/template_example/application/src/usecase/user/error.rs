use domain::user::email::UserEmail;
use lib::application::usecase_result;

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
    EmailAlreadyUsed(UserEmail),
}

usecase_result!(User);
