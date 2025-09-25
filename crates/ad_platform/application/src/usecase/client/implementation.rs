use async_trait::async_trait;
use domain::client::{Client, UpsertClient};
use lib::domain::Id;
use tracing::instrument;

use crate::{
    repository::{RepositoriesModuleExt, client::ClientRepository as _},
    service::ServicesModuleExt,
    usecase::{
        UseCase,
        client::{ClientUseCase, error::ClientUseCaseError},
    },
};

#[async_trait]
impl<R, S> ClientUseCase<R, S> for UseCase<R, S, Client>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    #[instrument(name = "ClientUseCase::bulk_upsert", skip_all)]
    async fn bulk_upsert(
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
    async fn find_by_id(
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
