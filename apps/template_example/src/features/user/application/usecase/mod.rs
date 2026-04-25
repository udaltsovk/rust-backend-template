use lib::{application::application_result, domain::Id};

pub use self::{
    authorize::AuthorizeUserUsecase,
    create::CreateUserUsecase,
    find_by_id::FindUserByIdUsecase,
    get_by_id::GetUserByIdUsecase,
};
use crate::{
    features::user::domain::User,
    shared::domain::email::Email,
};

mod authorize;
mod create;
mod find_by_id;
mod get_by_id;

pub trait UserUseCases = AuthorizeUserUsecase
    + CreateUserUsecase
    + FindUserByIdUsecase
    + GetUserByIdUsecase;

#[derive(thiserror::Error, Debug)]
pub enum UserUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error("user with the specified email already exists")]
    EmailAlreadyUsed(Email),

    #[error("user with the specified email does not exist")]
    NotFoundByEmail(Email),

    #[error("user with the specified id does not exist")]
    NotFoundById(Id<User>),

    #[error("invalid password")]
    InvalidPassword,
}

application_result!(UserUseCase);
