use async_trait::async_trait;
use domain::client::{Client, UpsertClient};
use lib::domain::Id;

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::client::error::ClientUseCaseError,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait ClientUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn bulk_upsert(
        &self,
        source: &[UpsertClient],
    ) -> Result<Vec<Client>, ClientUseCaseError<R, S>>;

    async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> Result<Option<Client>, ClientUseCaseError<R, S>>;
}
