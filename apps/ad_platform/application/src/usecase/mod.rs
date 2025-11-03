use std::marker::PhantomData;

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

pub mod client;

#[derive(Clone)]
pub struct UseCase<R, S, T>
where
    R: RepositoriesModuleExt + Send + Sync,
    S: ServicesModuleExt + Send + Sync,
{
    #[allow(unused)]
    repositories: R,
    #[allow(unused)]
    services: S,
    _entity: PhantomData<T>,
}

impl<R, S, T> UseCase<R, S, T>
where
    R: RepositoriesModuleExt + Send + Sync,
    S: ServicesModuleExt + Send + Sync,
{
    pub fn new(repositories: R, services: S) -> Self {
        Self {
            repositories,
            services,
            _entity: PhantomData,
        }
    }
}
