use domain::session::Session;
use lib::{application::application_result, domain::Id};

mod create;
mod get_from_token;

pub use create::CreateSessionUsecase;
pub use get_from_token::GetSessionFromTokenUsecase;

#[derive(thiserror::Error, Debug)]
pub enum SessionUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error("session with the specified id does not exist")]
    NotFound(Id<Session>),
}

application_result!(SessionUseCase);
