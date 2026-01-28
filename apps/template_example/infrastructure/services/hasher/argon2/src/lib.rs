use anyhow::{Context as _, Result};
use application::service::hasher::{HasherService, Password, PasswordHash};
use argon2::{
    Algorithm, Argon2, Params, ParamsBuilder, PasswordHasher as _,
    PasswordVerifier as _, Version,
    password_hash::{SaltString, rand_core::OsRng},
};
use lib::{instrument_all, tap::Pipe as _};
use redact::Secret;

#[derive(Clone)]
pub struct Argon2Service {
    hasher: Argon2<'static>,
}

#[instrument_all]
impl HasherService for Argon2Service {
    fn hash(&self, data: &Password) -> Result<PasswordHash> {
        self.hasher
            .hash_password(
                data.as_ref().expose_secret().as_bytes(),
                &Self::gen_salt(),
            )
            .map(|hashed| {
                hashed.to_string().pipe(Secret::new).pipe(PasswordHash)
            })
            .context("while hashing password with argon")
    }

    fn verify(
        &self,
        data: &Password,
        original_hash: &PasswordHash,
    ) -> Result<()> {
        self.hasher
            .verify_password(
                data.as_ref().expose_secret().as_bytes(),
                &original_hash
                    .0
                    .expose_secret()
                    .pipe(|hash| argon2::PasswordHash::new(hash))?,
            )
            .context("while verifying password with argon")
    }
}

impl Argon2Service {
    fn gen_salt() -> SaltString {
        SaltString::generate(&mut OsRng)
    }
}

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

    #[must_use]
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
