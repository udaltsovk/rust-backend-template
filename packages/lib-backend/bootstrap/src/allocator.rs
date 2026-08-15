#[macro_export]
macro_rules! configure_allocator {
    ($conf:literal) => {
        #[global_allocator]
        static ALLOCATOR: mimalloc::MiMalloc =
            mimalloc::MiMalloc;
    };
    () => {
        configure_allocator!(c"");
    };
}
