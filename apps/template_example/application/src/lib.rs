#![feature(bool_to_result, trait_alias)]

use crate::{repository::Repositories, service::Services};

pub mod repository;
pub mod service;
pub mod usecase;

pub trait Application = Repositories + Services + Clone + Send + Sync + 'static;
