use domain::session::Session;
use lib::{application::usecase_result, domain::Id};

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

#[derive(thiserror::Error, Debug)]
pub enum SessionUseCaseError<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    #[error("Repository error: {0}")]
    Repository(R::Error),

    #[error(transparent)]
    Service(S::Error),

    #[error("session with id `{0}` does not exist")]
    NotFound(Id<Session>),
}

usecase_result!(Session);
