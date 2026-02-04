use anyhow::Result;
pub use domain::password::{Password, PasswordHash};

pub trait HasherService {
    fn hash(&self, data: &Password) -> Result<PasswordHash>;

    fn verify(
        &self,
        data: &Password,
        original_hash: Option<&PasswordHash>,
    ) -> Result<()>;
}
