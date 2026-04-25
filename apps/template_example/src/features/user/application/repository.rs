use entrait::entrait;
use lib::{anyhow::Result, async_trait, domain::Id};

use crate::{
    features::user::domain::{CreateUser, User},
    shared::domain::{
        email::Email, password::PasswordHash,
    },
};

#[entrait(
    UserRepositoryImpl,
    delegate_by=DelegateUserRepository
)]
#[async_trait]
pub trait UserRepository {
    async fn create_user(
        &self,
        id: Id<User>,
        source: CreateUser,
        password_hash: PasswordHash,
    ) -> Result<User>;

    async fn find_user_by_id(
        &self,
        id: Id<User>,
    ) -> Result<Option<User>>;

    async fn find_user_by_email(
        &self,
        email: &Email,
    ) -> Result<Option<User>>;
}
