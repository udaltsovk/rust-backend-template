use std::fmt::Debug;

pub trait HasherService {
    type AdapterError: Debug + Send + Sync;

    fn hash(&self, data: &[u8]) -> Result<String, Self::AdapterError>;

    fn verify(
        &self,
        data: &[u8],
        original_hash: &str,
    ) -> Result<(), Self::AdapterError>;
}
