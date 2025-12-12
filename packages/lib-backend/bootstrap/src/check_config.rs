#[macro_export]
macro_rules! check_config {
    ($config: ident) => {{
        if !$config::check_values() && cfg!(not(debug_assertions)) {
            panic!("Not all environment variables are set!");
        }
    }};
}
