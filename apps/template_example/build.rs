use std::env;

fn main() {
    println!(
        "cargo::rustc-env=COMPILATION_PROFILE={}",
        env::var("PROFILE").as_deref().unwrap_or("unknown")
    );
}
