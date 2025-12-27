use domain::client::{Client, UpsertClient};
use lib::{async_trait, domain::Id};

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::client::error::ClientUseCaseResult,
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
    ) -> ClientUseCaseResult<R, S, Vec<Client>>;

    async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> ClientUseCaseResult<R, S, Option<Client>>;
}
