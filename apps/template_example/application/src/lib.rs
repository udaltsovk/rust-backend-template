#![feature(bool_to_result)]

use crate::{repository::Repositories, service::Services};

pub mod repository;
pub mod service;
pub mod usecase;

pub trait Application:
    Repositories + Services + Clone + Send + Sync + 'static
{
}

impl<T> Application for T where
    T: Repositories + Services + Clone + Send + Sync + 'static
{
}
