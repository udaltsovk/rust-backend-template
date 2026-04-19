use self::application::{UserRepositories, UserUseCases};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub trait UserFeature = UserRepositories + UserUseCases;
