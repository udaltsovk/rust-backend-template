#[macro_export]
macro_rules! bootstrap {
    ($modules: ident, $($bootstrapper: tt),*) => {
        tokio::join!(
            $($bootstrapper::bootstrap($modules.clone())),*
        );
    };
}
