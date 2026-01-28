use anyhow::Result;
use domain::session::Session;
use redact::Secret;

pub trait TokenService {
    fn generate(&self, session: Session) -> Result<Secret<String>>;

    fn parse(&self, token: Secret<&str>) -> Result<Session>;
}
