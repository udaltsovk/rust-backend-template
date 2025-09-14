use std::fmt::Debug;

use domain::session::Session;

pub trait TokenService {
    type AdapterError: Debug + Send + Sync;

    fn generate(&self, session: Session) -> Result<String, Self::AdapterError>;

    fn parse(&self, token: &str) -> Result<Session, Self::AdapterError>;
}
