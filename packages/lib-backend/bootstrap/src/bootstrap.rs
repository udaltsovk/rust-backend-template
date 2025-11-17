#[macro_export]
macro_rules! bootstrap {
    ($app_crate: ident, [$($bootstrapper: tt),*], $modules_fut: expr) => {
        (async || {
            use $app_crate::{bootstrappers::BootstrapperExt as _, config};

            $crate::check_config!(config);

            let modules = $modules_fut.await;
            tokio::join!(
                $($bootstrapper::bootstrap(modules.clone())),*
            );
        })()
    };
}
