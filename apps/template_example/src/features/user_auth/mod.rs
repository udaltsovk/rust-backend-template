use self::application::{
    AuthRepositories, AuthServices, AuthUseCases,
};

pub mod application;
pub mod domain;
pub mod infrastructure;
pub mod presentation;

pub trait UserAuthFeature =
    AuthRepositories + AuthServices + AuthUseCases;
