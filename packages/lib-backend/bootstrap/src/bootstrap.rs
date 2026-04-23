#[macro_export]
macro_rules! bootstrap {
    ($_app_crate: ident, [], $_modules_fut: expr) => {
      const {
           panic!("`bootstrap!` can't be called with empty bootstrapper array!");
      }
    };
    ($app_crate: ident, [$($bootstrapper: tt($config_field: expr)),*], $modules_fut: expr) => {
        async {
            use $app_crate::modules::BootstrapperExt as _;


            let modules = $crate::entrait::Impl::new($modules_fut.await);
            tokio::join!(
                $($bootstrapper::bootstrap($config_field, &modules)),*
            );
        }
    };
}
