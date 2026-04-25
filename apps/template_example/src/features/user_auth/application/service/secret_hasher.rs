use entrait::entrait;
use lib::anyhow::Result;

pub use crate::shared::domain::password::{
    Password, PasswordHash,
};

#[entrait(
    SecretHasherServiceImpl,
    delegate_by=DelegateSecretHasherService
)]
pub trait SecretHasherService {
    fn hash_secret(
        &self,
        data: &Password,
    ) -> Result<PasswordHash>;

    fn verify_secret(
        &self,
        data: &Password,
        original_hash: Option<&PasswordHash>,
    ) -> Result<()>;
}
