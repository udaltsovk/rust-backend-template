use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

#[derive(thiserror::Error, Debug)]
pub enum ClientUseCaseError<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    #[error("Repository error: {0}")]
    Repository(R::Error),

    #[error(transparent)]
    Service(S::Error),
}
