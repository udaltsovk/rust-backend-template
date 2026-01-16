use domain::session::{Session, entity::SessionEntity};
use lib::async_trait;

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::session::error::SessionUseCaseResult,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait SessionUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn create(
        &self,
        entity: SessionEntity,
    ) -> SessionUseCaseResult<R, S, String>;

    async fn get_from_token(
        &self,
        token: &str,
    ) -> SessionUseCaseResult<R, S, Session>;
}
