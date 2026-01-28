use domain::{
    session::CreateSession,
    user::{CreateUser, User},
};
use lib::{async_trait, domain::Id};

use crate::usecase::user::error::UserUseCaseResult;

pub mod error;
pub mod implementation;

#[async_trait]
pub trait UserUseCase {
    async fn create(&self, source: CreateUser) -> UserUseCaseResult<User>;

    async fn authorize(&self, source: CreateSession)
    -> UserUseCaseResult<User>;

    async fn find_by_id(&self, id: Id<User>)
    -> UserUseCaseResult<Option<User>>;

    async fn get_by_id(&self, id: Id<User>) -> UserUseCaseResult<User>;
}
