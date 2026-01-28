use crate::repository::{session::SessionRepository, user::UserRepository};

pub mod session;
pub mod user;

pub trait RepositoriesModuleExt: Send + Sync {
    fn user_repository(&self) -> &dyn UserRepository;

    fn session_repository(&self) -> &dyn SessionRepository;
}
