#[macro_export]
macro_rules! extractor {
    ($from:ident, $name: ident, $rejection: ident) => {
        #[derive(Debug, Clone, $from)]
        #[from_request(via(axum::extract::$name), rejection($rejection))]
        pub struct $name<T>(pub T);
    };
}
