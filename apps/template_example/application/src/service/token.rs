use domain::session::Session;
use lib::{anyhow::Result, redact::Secret};

pub trait TokenService {
    fn generate(&self, session: Session) -> Result<Secret<String>>;

    fn parse(&self, token: Secret<&str>) -> Result<Session>;
}
