use kernel::{
    application::{repository::RepositoriesModuleExt, usecase::UseCase},
    domain::client::Client,
};

pub trait ModulesExt: Clone + Send + Sync + 'static {
    type RepositoriesModule: RepositoriesModuleExt;

    fn client_usecase(&self) -> &UseCase<Self::RepositoriesModule, Client>;
}
