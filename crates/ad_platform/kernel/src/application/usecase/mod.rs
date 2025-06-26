use std::marker::PhantomData;

use crate::application::repository::RepositoriesModuleExt;

pub mod client;
pub mod error;

#[derive(Clone)]
pub struct UseCase<R, T>
where
    R: RepositoriesModuleExt + Send + Sync,
{
    repositories: R,
    _entity: PhantomData<T>,
}
impl<R, T> UseCase<R, T>
where
    R: RepositoriesModuleExt + Send + Sync,
{
    pub fn new(repositories: R) -> Self {
        Self {
            repositories,
            _entity: PhantomData,
        }
    }
}
