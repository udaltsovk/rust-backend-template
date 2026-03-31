use application::service::token::TokenServiceImpl;
use domain::session::Session;
use entrait::entrait;
use jsonwebtoken::{Algorithm, Header, Validation, decode, encode};
pub use jsonwebtoken::{DecodingKey, EncodingKey};
use lib::{
    anyhow::{Context as _, Result},
    application::di::Has,
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

#[entrait(ref)]
#[instrument_all]
impl TokenServiceImpl for JwtService {
    fn generate_token<App>(
        app: &App,
        session: Session,
    ) -> Result<Secret<String>>
    where
        App: Has<Self>,
    {
        encode(
            &Header::new(Algorithm::HS256),
            &Claims::from(session),
            &app.get_dependency().encoding_key,
        )
        .map(Secret::new)
        .context("while encoding jwt")
    }

    fn parse_token<App>(app: &App, token: Secret<&str>) -> Result<Session>
    where
        App: Has<Self>,
    {
        decode::<Claims>(
            token.expose_secret(),
            &app.get_dependency().decoding_key,
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
