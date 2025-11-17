#[macro_export]
macro_rules! check_config {
    ($config: ident) => {{
        if !$config::check_values() && env!("COMPILATION_PROFILE") == "release"
        {
            panic!("Not all environment variables are set!");
        }
    }};
}
