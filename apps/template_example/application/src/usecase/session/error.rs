use domain::session::Session;
use lib::{application::application_result, domain::Id};

#[derive(thiserror::Error, Debug)]
pub enum SessionUseCaseError {
    #[error(transparent)]
    Infrastructure(#[from] lib::anyhow::Error),

    #[error("session with the specified id does not exist")]
    NotFound(Id<Session>),
}

application_result!(SessionUseCase);
