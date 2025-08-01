use crate::{
    application::{
        repository::{RepositoriesModuleExt, client::ClientRepository},
        usecase::{UseCase, error::UseCaseError},
    },
    domain::{
        Id,
        client::{Client, UpsertClient},
    },
};

impl<R> UseCase<R, Client>
where
    R: RepositoriesModuleExt,
{
    pub type Error =
        UseCaseError<<R::ClientRepo as ClientRepository>::AdapterError>;

    pub async fn bulk_upsert(
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

    pub async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> Result<Option<Client>, Self::Error> {
        let result =
            self.repositories.client_repository().find_by_id(id).await?;
        Ok(result)
    }
}
