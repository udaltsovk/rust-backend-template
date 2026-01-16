use lib::application::usecase_struct;

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

pub mod session;
pub mod user;

usecase_struct!(RepositoriesModuleExt, ServicesModuleExt);
