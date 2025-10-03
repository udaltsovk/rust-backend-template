use application::service::token::TokenService;
use domain::session::Session;
use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use lib::instrument_all;
use tap::{Conv as _, Pipe as _, Tap as _};
use uuid::Uuid;

use crate::claims::Claims;

mod claims;

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

#[instrument_all("JwtService")]
impl TokenService for JwtService {
    type AdapterError = jsonwebtoken::errors::Error;

    fn generate(&self, session: Session) -> Result<String, Self::AdapterError> {
        let entity_id: Uuid = session.entity.clone().into();
        let header = Header::new(Algorithm::RS256).tap_mut(|header| {
            header.kid = entity_id.to_string().pipe(Some);
        });
        encode(&header, &Claims::from(session), &self.encoding_key)
    }

    fn parse(&self, token: &str) -> Result<Session, Self::AdapterError> {
        decode::<Claims>(token, &self.decoding_key, &Validation::default())?
            .claims
            .conv::<Session>()
            .pipe(Ok)
    }
}

#[instrument_all("JwtService")]
impl JwtService {
    pub fn new(secret: &str) -> Self {
        let secret = secret.as_bytes();
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }
}
