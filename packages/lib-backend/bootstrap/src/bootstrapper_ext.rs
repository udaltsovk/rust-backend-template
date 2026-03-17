#[macro_export]
macro_rules! bootstrapper_ext_trait {
    ($modules_ty: ty) => {
        #[lib::async_trait]
        pub trait BootstrapperExt {
            type Config: fromenv::__private::FromEnv;

            async fn bootstrap(
                config: &Self::Config,
                deps: ::entrait::Impl<$modules_ty>,
            );
        }
    };
}
