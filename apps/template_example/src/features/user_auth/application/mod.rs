pub mod repository;
pub mod service;
pub mod usecase;

pub trait AuthRepositories = repository::AuthRepositories;
pub trait AuthServices = service::AuthServices;
pub trait AuthUseCases = usecase::AuthUseCases;
