fn main() {
    println!(
        "cargo::rustc-env=COMPILATION_PROFILE={}",
        std::env::var("PROFILE").as_deref().unwrap_or("unknown")
    );
}
