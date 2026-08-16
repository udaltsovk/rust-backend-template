use async_trait::async_trait;

#[async_trait]
pub trait Bootstrapper {
    type Config: fromenv::__private::FromEnv;
    type Modules: Send + Sync;

    async fn bootstrap(
        config: &Self::Config,
        deps: &::entrait::Impl<Self::Modules>,
    );
}

#[macro_export]
macro_rules! bootstrap {
    ([], $_modules_fut: expr) => {
       compile_error!("`bootstrap!` can't be called with empty bootstrapper array!")
    };
    ([$($bootstrapper: tt($config_field: expr)),*], $modules_fut: expr) => {
        async {
            use $crate::Bootstrapper as _;


            let modules = $crate::entrait::Impl::new($modules_fut.await);
            tokio::join!(
                $($bootstrapper::bootstrap($config_field, &modules)),*
            );
        }
    };
}
