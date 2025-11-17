#[macro_export]
macro_rules! configure_jemalloc {
    ($conf:literal) => {
        #[cfg(not(target_env = "msvc"))]
        #[global_allocator]
        static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

        #[cfg(not(target_env = "msvc"))]
        #[unsafe(export_name = "malloc_conf")]
        pub static MALLOC_CONF: &std::ffi::CStr = $conf;
    };
    () => {
        configure_jemalloc!(c"prof:true,prof_active:true,lg_prof_sample:19");
    };
}
