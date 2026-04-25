use self::repository::UserRepository;
pub use self::usecase::UserUseCases;

pub mod repository;
pub mod usecase;

pub trait UserRepositories = UserRepository;
