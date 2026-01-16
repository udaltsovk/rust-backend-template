use domain::{
    session::CreateSession,
    user::{CreateUser, User},
};
use lib::{async_trait, domain::Id};

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
    async fn create(&self, source: CreateUser)
    -> UserUseCaseResult<R, S, User>;

    async fn authorize(
        &self,
        source: CreateSession,
    ) -> UserUseCaseResult<R, S, User>;

    async fn find_by_id(
        &self,
        id: Id<User>,
    ) -> UserUseCaseResult<R, S, Option<User>>;

    async fn get_by_id(&self, id: Id<User>) -> UserUseCaseResult<R, S, User>;
}
