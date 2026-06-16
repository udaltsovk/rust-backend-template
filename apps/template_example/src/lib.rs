#![feature(trait_alias, try_blocks)]

pub use self::config::AppConfig;

pub mod bootstrappers;
mod config;
pub mod features;
pub mod modules;
pub mod shared;
