use derive_where::derive_where;

use crate::{
    repository::{RepositoriesModuleExt, client::ClientRepository},
    service::{ServicesModuleExt, token::TokenService},
};

#[derive(thiserror::Error)]
#[derive_where(Debug)]
pub enum ClientUseCaseError<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    #[error(transparent)]
    Repository(<R::ClientRepo as ClientRepository>::AdapterError),
    #[error(transparent)]
    Service(<S::TokenService as TokenService>::AdapterError),
}
