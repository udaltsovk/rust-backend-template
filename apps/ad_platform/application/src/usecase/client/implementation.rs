use async_trait::async_trait;
use domain::client::{Client, UpsertClient};
use lib::{domain::Id, instrument_all};
use tap::Pipe as _;

use crate::{
    repository::{RepositoriesModuleExt, client::ClientRepository as _},
    service::ServicesModuleExt,
    usecase::{
        UseCase,
        client::{ClientUseCase, error::ClientUseCaseError},
    },
};

#[async_trait]
#[instrument_all("ClientUseCase")]
impl<R, S> ClientUseCase<R, S> for UseCase<R, S, Client>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn bulk_upsert(
        &self,
        source: &[UpsertClient],
    ) -> Result<Vec<Client>, ClientUseCaseError<R, S>> {
        self.repositories
            .client_repository()
            .bulk_upsert(source)
            .await
            .map_err(R::Error::from)
            .map_err(ClientUseCaseError::Repository)?
            .pipe(Ok)
    }

    async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> Result<Option<Client>, ClientUseCaseError<R, S>> {
        self.repositories
            .client_repository()
            .find_by_id(id)
            .await
            .map_err(R::Error::from)
            .map_err(ClientUseCaseError::Repository)?
            .pipe(Ok)
    }
}
