#![feature(trait_alias)]

#[doc(hidden)]
pub use {
    domain::validation::{ExternalInput, error::ValidationErrors},
    serde_json, tap,
};

pub mod errors;
pub mod extract;
pub mod health;
pub mod mask;
pub mod negotiate;
mod panic_handler;
pub mod request_id;
pub mod response;
pub mod routes;
pub mod startup;
pub mod tracing;
pub mod validation;
