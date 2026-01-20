use application::service::hasher::HasherService;
pub use argon2::password_hash::Error as Argon2AdapterError;
use argon2::{
    Algorithm, Argon2, Params, ParamsBuilder, PasswordHash,
    PasswordHasher as _, PasswordVerifier as _, Version,
    password_hash::{SaltString, rand_core::OsRng},
};
use lib::instrument_all;

pub struct Argon2Service {
    hasher: Argon2<'static>,
}

#[instrument_all]
impl HasherService for Argon2Service {
    type AdapterError = Argon2AdapterError;

    fn hash(&self, data: &[u8]) -> Result<String, Self::AdapterError> {
        self.hasher
            .hash_password(data, &Self::gen_salt())
            .map(|hashed| hashed.to_string())
    }

    fn verify(
        &self,
        data: &[u8],
        original_hash: &str,
    ) -> Result<(), Self::AdapterError> {
        self.hasher
            .verify_password(data, &PasswordHash::new(original_hash)?)
    }
}

#[instrument_all(level = "debug")]
impl Argon2Service {
    fn gen_salt() -> SaltString {
        SaltString::generate(&mut OsRng)
    }
}

#[instrument_all(level = "trace")]
impl Argon2Service {
    #[inline]
    fn params() -> Params {
        ParamsBuilder::new()
            .m_cost(19_456)
            .t_cost(1)
            .p_cost(4)
            .output_len(32)
            .build()
            .expect("hasher params should be valid")
    }

    pub fn new() -> Self {
        Self {
            hasher: Argon2::new(
                Algorithm::Argon2id,
                Version::V0x13,
                Self::params(),
            ),
        }
    }

    pub fn new_with_secret(secret: &'static [u8]) -> argon2::Result<Self> {
        Ok(Self {
            hasher: Argon2::new_with_secret(
                secret,
                Algorithm::Argon2id,
                Version::V0x13,
                Self::params(),
            )?,
        })
    }
}

impl Default for Argon2Service {
    fn default() -> Self {
        Self::new()
    }
}
