use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use crate::repository::client::ClientRepository;

pub mod client;

pub trait RepositoriesModuleExt: Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::ClientRepo as ClientRepository>::AdapterError>;

    type ClientRepo: ClientRepository + Send + Sync;

    fn client_repository(&self) -> Arc<Self::ClientRepo>;
}
