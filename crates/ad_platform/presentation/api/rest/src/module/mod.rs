use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::UseCase,
};
use domain::client::Client;

pub trait ModulesExt: Clone + Send + Sync + 'static {
    type RepositoriesModule: RepositoriesModuleExt;
    type ServicesModule: ServicesModuleExt;

    fn client_usecase(
        &self,
    ) -> &UseCase<Self::RepositoriesModule, Self::ServicesModule, Client>;
}
