use application::service::token::TokenServiceImpl;
use domain::session::Session;
use entrait::entrait;
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

#[entrait(pub HasJwtService)]
fn jwt_service(service: &JwtService) -> &JwtService {
    service
}

#[entrait(ref)]
#[instrument_all]
impl TokenServiceImpl for JwtService {
    fn generate_token<App>(
        app: &App,
        session: Session,
    ) -> Result<Secret<String>>
    where
        App: HasJwtService,
    {
        encode(
            &Header::new(Algorithm::HS256),
            &Claims::from(session),
            &app.jwt_service().encoding_key,
        )
        .map(Secret::new)
        .context("while encoding jwt")
    }

    fn parse_token<App>(app: &App, token: Secret<&str>) -> Result<Session>
    where
        App: HasJwtService,
    {
        decode::<Claims>(
            token.expose_secret(),
            &app.jwt_service().decoding_key,
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
