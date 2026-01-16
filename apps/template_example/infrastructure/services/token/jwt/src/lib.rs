use application::service::token::TokenService;
use domain::session::Session;
use jsonwebtoken::{Algorithm, Header, Validation, decode, encode};
pub use jsonwebtoken::{
    DecodingKey, EncodingKey, errors::Error as JwtAdapterError,
};
use lib::{
    instrument_all,
    tap::{Conv as _, Pipe as _},
};

use crate::claims::Claims;

mod claims;

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

#[instrument_all]
impl TokenService for JwtService {
    type AdapterError = JwtAdapterError;

    fn generate(&self, session: Session) -> Result<String, Self::AdapterError> {
        encode(
            &Header::new(Algorithm::HS256),
            &Claims::from(session),
            &self.encoding_key,
        )
    }

    fn parse(&self, token: &str) -> Result<Session, Self::AdapterError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())?
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
