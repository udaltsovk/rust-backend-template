#[cfg(any(
    feature = "opentelemetry-http-proto",
    feature = "opentelemetry-http-json",
    feature = "opentelemetry-grpc-tonic"
))]
pub use opentelemetry;
