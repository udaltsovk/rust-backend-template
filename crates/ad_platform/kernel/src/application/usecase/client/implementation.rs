use async_trait::async_trait;

use crate::{
    application::{
        repository::{RepositoriesModuleExt, client::ClientRepository},
        usecase::{UseCase, client::ClientUseCase, error::UseCaseError},
    },
    domain::{
        Id,
        client::{Client, UpsertClient},
    },
};

#[async_trait]
impl<R> ClientUseCase for UseCase<R, Client>
where
    R: RepositoriesModuleExt,
{
    type Error =
        UseCaseError<<R::ClientRepo as ClientRepository>::AdapterError>;

    async fn bulk_upsert(
        &self,
        source: &[UpsertClient],
    ) -> Result<Vec<Client>, Self::Error> {
        let result = self
            .repositories
            .client_repository()
            .bulk_upsert(source)
            .await?;
        Ok(result)
    }

    async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> Result<Option<Client>, Self::Error> {
        let result =
            self.repositories.client_repository().find_by_id(id).await?;
        Ok(result)
    }
}
