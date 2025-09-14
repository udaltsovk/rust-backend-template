use lib::kernel::domain::Id;
use tracing::instrument;

use crate::{
    application::{
        repository::{RepositoriesModuleExt, client::ClientRepository},
        service::ServicesModuleExt,
        usecase::{UseCase, client::error::ClientUseCaseError},
    },
    domain::client::{Client, UpsertClient},
};

pub mod error;

impl<R, S> UseCase<R, S, Client>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    #[instrument(name = "ClientUseCase::bulk_upsert", skip_all)]
    pub async fn bulk_upsert(
        &self,
        source: &[UpsertClient],
    ) -> Result<Vec<Client>, ClientUseCaseError<R, S>> {
        let result = self
            .repositories
            .client_repository()
            .bulk_upsert(source)
            .await
            .map_err(ClientUseCaseError::Repository)?;
        Ok(result)
    }

    #[instrument(name = "ClientUseCase::find_by_id", skip_all)]
    pub async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> Result<Option<Client>, ClientUseCaseError<R, S>> {
        let result = self
            .repositories
            .client_repository()
            .find_by_id(id)
            .await
            .map_err(ClientUseCaseError::Repository)?;
        Ok(result)
    }
}
