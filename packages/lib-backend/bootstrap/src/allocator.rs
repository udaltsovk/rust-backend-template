#[macro_export]
macro_rules! configure_allocator {
    () => {
        #[global_allocator]
        static ALLOCATOR: $crate::mimalloc::MiMalloc =
            $crate::mimalloc::MiMalloc;
    };
}
