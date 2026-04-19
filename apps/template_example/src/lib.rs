#![feature(bool_to_result, trait_alias, try_blocks)]

pub use self::config::AppConfig;
use self::features::{
    user::UserFeature, user_auth::UserAuthFeature,
};

pub mod bootstrappers;
mod config;
pub mod features;
pub mod modules;
pub mod shared;

pub trait Application = Clone
    + Send
    + Sync
    + UserFeature
    + UserAuthFeature
    + 'static;
