use domain::{email::Email, user::User};
use lib::{application::application_result, domain::Id};

mod authorize;
mod create;
mod find_by_id;
mod get_by_id;

pub use authorize::AuthorizeUserUsecase;
pub use create::CreateUserUsecase;
pub use find_by_id::FindUserByIdUsecase;
pub use get_by_id::GetUserByIdUsecase;

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
