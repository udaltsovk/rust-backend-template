use domain::{
    email::Email,
    password::PasswordHash,
    user::{CreateUser, User},
};
use lib::{anyhow::Result, async_trait, domain::Id};

#[async_trait]
pub trait UserRepository {
    async fn create(
        &self,
        id: Id<User>,
        source: CreateUser,
        password_hash: PasswordHash,
    ) -> Result<User>;

    async fn find_by_id(&self, id: Id<User>) -> Result<Option<User>>;

    async fn find_by_email(&self, email: &Email) -> Result<Option<User>>;
}
