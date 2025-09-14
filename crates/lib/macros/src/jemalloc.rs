#[macro_export]
macro_rules! configure_jemalloc {
    ($conf:literal) => {
        #[cfg(target_os = "linux")]
        #[global_allocator]
        static ALLOC: tikv_jemallocator::Jemalloc = tikv_jemallocator::Jemalloc;

        #[cfg(target_os = "linux")]
        #[unsafe(export_name = "malloc_conf")]
        pub static MALLOC_CONF: &std::ffi::CStr = $conf;
    };
    () => {
        configure_jemalloc!(c"prof:true,prof_active:true,lg_prof_sample:19");
    };
}
