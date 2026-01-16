use application::{
    repository::RepositoriesModuleExt,
    service::ServicesModuleExt,
    usecase::{session::SessionUseCase, user::UserUseCase},
};
use lib::presentation::usecase_impl_type;

pub trait ModulesExt: Clone + Send + Sync + 'static {
    type RepositoriesModule: RepositoriesModuleExt;
    type ServicesModule: ServicesModuleExt;

    fn user_usecase(
        &self,
    ) -> &impl UserUseCase<Self::RepositoriesModule, Self::ServicesModule>;

    fn session_usecase(
        &self,
    ) -> &impl SessionUseCase<Self::RepositoriesModule, Self::ServicesModule>;
}

usecase_impl_type!();
