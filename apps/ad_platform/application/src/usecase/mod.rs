use lib::application::usecase_struct;

use crate::{repository::RepositoriesModuleExt, service::ServicesModuleExt};

pub mod client;

usecase_struct!(RepositoriesModuleExt, ServicesModuleExt);
