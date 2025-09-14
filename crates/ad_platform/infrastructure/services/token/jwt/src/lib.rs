use jsonwebtoken::{
    Algorithm, DecodingKey, EncodingKey, Header, Validation, decode, encode,
};
use kernel::{
    application::service::token::TokenService, domain::session::Session,
};
use tracing::instrument;
use uuid::Uuid;

use crate::claims::Claims;

mod claims;

pub struct JwtService {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}
impl TokenService for JwtService {
    type AdapterError = jsonwebtoken::errors::Error;

    #[instrument(name = "JwtService::generate", skip_all)]
    fn generate(&self, session: Session) -> Result<String, Self::AdapterError> {
        let mut header = Header::new(Algorithm::RS256);
        let entity_id: Uuid = session.entity.clone().into();
        header.kid = Some(entity_id.to_string());
        encode(&header, &Claims::from(session), &self.encoding_key)
    }

    #[instrument(name = "JwtService::parse", skip_all)]
    fn parse(&self, token: &str) -> Result<Session, Self::AdapterError> {
        let claims: Claims =
            decode(token, &self.decoding_key, &Validation::default())?.claims;
        Ok(claims.into())
    }
}
impl JwtService {
    #[tracing::instrument(name = "JwtService::new", skip_all, level = "trace")]
    pub fn new(secret: &str) -> Self {
        let secret = secret.as_bytes();
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }
}
