use lib::{application::application_result, domain::Id};

mod create;
mod get_from_token;

pub use self::{
    create::CreateSessionUsecase,
    get_from_token::GetSessionFromTokenUsecase,
};
use crate::features::user_auth::domain::session::Session;

#[derive(thiserror::Error, Debug)]
pub enum SessionUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error("session with the specified id does not exist")]
    NotFound(Id<Session>),
}

application_result!(SessionUseCase);
