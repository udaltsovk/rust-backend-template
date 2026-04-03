use domain::session::Session;
use entrait::entrait;
use lib::{anyhow::Result, redact::Secret};

#[entrait(
    TokenServiceImpl,
    delegate_by=DelegateTokenService
)]
pub trait TokenService {
    fn generate_token(&self, session: Session) -> Result<Secret<String>>;

    fn parse_token(&self, token: Secret<&str>) -> Result<Session>;
}
