use domain::{email::Email, user::User};
use lib::{application::application_result, domain::Id};

#[derive(thiserror::Error, Debug)]
pub enum UserUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] anyhow::Error),

    #[error("user with the specified email already exists")]
    EmailAlreadyUsed(Email),

    #[error("user with the specified email does not exist")]
    NotFoundByEmail { email: Email, from_auth: bool },

    #[error("user with the specified id does not exist")]
    NotFoundById(Id<User>),

    #[error("invalid password")]
    InvalidPassword,
}

application_result!(UserUseCase);
