pub use domain::password::{Password, PasswordHash};
use entrait::entrait;
use lib::anyhow::Result;

#[entrait(SecretHasherServiceImpl, delegate_by=ref)]
pub trait SecretHasherService {
    fn hash_secret(&self, data: &Password) -> Result<PasswordHash>;

    fn verify_secret(
        &self,
        data: &Password,
        original_hash: Option<&PasswordHash>,
    ) -> Result<()>;
}
