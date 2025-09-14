use std::fmt::Debug;

use async_trait::async_trait;
use lib::kernel::domain::Id;

use crate::domain::client::{Client, UpsertClient};

#[async_trait]
pub trait ClientRepository {
    type AdapterError: Debug + Send + Sync;

    async fn bulk_upsert(
        &self,
        source: &[UpsertClient],
    ) -> Result<Vec<Client>, Self::AdapterError>;

    async fn find_by_id(
        &self,
        id: Id<Client>,
    ) -> Result<Option<Client>, Self::AdapterError>;
}
