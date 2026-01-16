use std::fmt::{Debug, Display};

use crate::repository::{session::SessionRepository, user::UserRepository};

pub mod session;
pub mod user;

pub trait RepositoriesModuleExt: Clone + Send + Sync {
    type Error: Debug
        + Display
        + From<<Self::UserRepo as UserRepository>::AdapterError>
        + From<<Self::SessionRepo as SessionRepository>::AdapterError>;

    type UserRepo: UserRepository + Send + Sync;
    fn user_repository(&self) -> &Self::UserRepo;

    type SessionRepo: SessionRepository + Send + Sync;
    fn session_repository(&self) -> &Self::SessionRepo;
}
