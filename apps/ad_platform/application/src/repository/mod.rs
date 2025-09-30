use std::sync::Arc;

use crate::repository::client::ClientRepository;

pub mod client;

pub trait RepositoriesModuleExt: Send + Sync {
    type ClientRepo: ClientRepository + Send + Sync;

    fn client_repository(&self) -> Arc<Self::ClientRepo>;
}
