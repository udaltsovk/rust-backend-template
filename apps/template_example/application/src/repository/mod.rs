use crate::repository::{session::SessionRepository, user::UserRepository};

pub mod session;
pub mod user;

pub trait Repositories = UserRepository + SessionRepository;
