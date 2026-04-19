use entrait::entrait;
use lib::{anyhow::Result, redact::Secret};

use crate::features::user_auth::domain::session::Session;

#[entrait(
    TokenServiceImpl,
    delegate_by=DelegateTokenService
)]
pub trait TokenService {
    fn generate_token(
        &self,
        session: Session,
    ) -> Result<Secret<String>>;

    fn parse_token(
        &self,
        token: Secret<&str>,
    ) -> Result<Session>;
}
