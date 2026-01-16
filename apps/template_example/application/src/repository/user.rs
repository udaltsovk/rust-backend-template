use std::fmt::Debug;

use domain::{
    email::Email,
    password::PasswordHash,
    user::{CreateUser, User},
};
use lib::{async_trait, domain::Id};

#[async_trait]
pub trait UserRepository {
    type AdapterError: Debug + Send + Sync;

    async fn create(
        &self,
        id: Id<User>,
        source: CreateUser,
        password_hash: String,
    ) -> Result<User, Self::AdapterError>;

    async fn find_by_id(
        &self,
        id: Id<User>,
    ) -> Result<Option<(User, PasswordHash)>, Self::AdapterError>;

    async fn find_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<(User, PasswordHash)>, Self::AdapterError>;
}
