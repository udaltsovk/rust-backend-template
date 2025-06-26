use std::sync::Arc;

use kernel::{
    application::repository::RepositoriesModuleExt, domain::client::Client,
};

use crate::{Db, repository::DatabaseRepositoryImpl};

#[derive(Clone)]
pub struct RepositoriesModule {
    client_repository: Arc<DatabaseRepositoryImpl<Client>>,
}
impl RepositoriesModule {
    pub fn new(db: &Db) -> Self {
        let client_repository = Arc::new(DatabaseRepositoryImpl::new(db));
        Self {
            client_repository,
        }
    }
}
impl RepositoriesModuleExt for RepositoriesModule {
    type ClientRepo = DatabaseRepositoryImpl<Client>;

    fn client_repository(&self) -> Arc<Self::ClientRepo> {
        self.client_repository.clone()
    }
}
