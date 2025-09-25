use application::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::client::ClientUseCase,
};

pub trait ModulesExt: Clone + Send + Sync + 'static {
    type RepositoriesModule: RepositoriesModuleExt;
    type ServicesModule: ServicesModuleExt;

    fn client_usecase(
        &self,
    ) -> &impl ClientUseCase<Self::RepositoriesModule, Self::ServicesModule>;
}
