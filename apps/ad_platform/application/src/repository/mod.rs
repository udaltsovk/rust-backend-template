use std::fmt::{Debug, Display};

use crate::repository::client::ClientRepository;

pub mod client;

pub trait RepositoriesModuleExt: Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::ClientRepo as ClientRepository>::AdapterError>;

    type ClientRepo: ClientRepository + Send + Sync;

    fn client_repository(&self) -> &Self::ClientRepo;
}
