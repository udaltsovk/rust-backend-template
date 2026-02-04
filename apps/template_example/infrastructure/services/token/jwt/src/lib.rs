use application::service::token::TokenService;
use domain::session::Session;
use jsonwebtoken::{Algorithm, Header, Validation, decode, encode};
pub use jsonwebtoken::{DecodingKey, EncodingKey};
use lib::{
    anyhow::{Context as _, Result},
    instrument_all,
    redact::Secret,
    tap::{Conv as _, Pipe as _},
};

use crate::claims::Claims;

mod claims;

#[derive(Clone)]
pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

#[instrument_all]
impl TokenService for JwtService {
    fn generate(&self, session: Session) -> Result<Secret<String>> {
        encode(
            &Header::new(Algorithm::HS256),
            &Claims::from(session),
            &self.encoding_key,
        )
        .map(Secret::new)
        .context("while encoding jwt")
    }

    fn parse(&self, token: Secret<&str>) -> Result<Session> {
        decode::<Claims>(
            token.expose_secret(),
            &self.decoding_key,
            &Validation::default(),
        )
        .context("while decoding jwt")?
        .claims
        .conv::<Session>()
        .pipe(Ok)
    }
}

impl JwtService {
    #[must_use]
    pub const fn new(
        encoding_key: EncodingKey,
        decoding_key: DecodingKey,
    ) -> Self {
        Self {
            encoding_key,
            decoding_key,
        }
    }
}
