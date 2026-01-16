use domain::user::{CreateUser, User};
use lib::async_trait;

use crate::{
    repository::RepositoriesModuleExt, service::ServicesModuleExt,
    usecase::user::error::UserUseCaseResult,
};

pub mod error;
pub mod implementation;

#[async_trait]
pub trait UserUseCase<R, S>
where
    R: RepositoriesModuleExt,
    S: ServicesModuleExt,
{
    async fn register(
        &self,
        source: CreateUser,
    ) -> UserUseCaseResult<R, S, User>;
}
