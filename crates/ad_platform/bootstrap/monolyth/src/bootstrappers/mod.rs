use async_trait::async_trait;

use crate::Modules;

pub mod rest_api;

#[async_trait]
pub trait BootstraperExt {
    async fn bootstrap(modules: Modules);
}
