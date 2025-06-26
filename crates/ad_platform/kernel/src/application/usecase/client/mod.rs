use async_trait::async_trait;

use crate::domain::{
    Id,
    client::{Client, UpsertClient},
};

pub mod implementation;

#[async_trait]
pub trait ClientUseCase {
    type Error;

    async fn bulk_upsert(
        &self,
        source: &[UpsertClient],
    ) -> Result<Vec<Client>, Self::Error>;

    async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> Result<Option<Client>, Self::Error>;
}
