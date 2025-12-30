use std::{fs, io, marker::PhantomData};

use fromenv::__private::FromEnv;
#[cfg(feature = "openapi")]
use utoipa::openapi::OpenApi;

#[derive(thiserror::Error, Debug)]
pub enum MetadataSaverError {
    #[cfg(feature = "openapi")]
    #[error("Failed to serialize OpenAPI: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Failed to save metadata: {0}")]
    Save(#[from] io::Error),
}

pub type MetadataSaverResult = Result<(), MetadataSaverError>;

pub trait MetadataSaver {
    fn save_as(&self, name: &str) -> MetadataSaverResult;
}

pub struct DotenvExample<C>(PhantomData<C>)
where
    C: FromEnv;

impl<C> Default for DotenvExample<C>
where
    C: FromEnv,
{
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<C> MetadataSaver for DotenvExample<C>
where
    C: FromEnv,
{
    fn save_as(&self, name: &str) -> MetadataSaverResult {
        let mut dotenv_example = String::new();
        C::requirements(&mut dotenv_example);

        let path = format!("./assets/dotenv/{name}.env.example");
        fs::write(&path, &dotenv_example)?;

        Ok(())
    }
}

#[cfg(feature = "openapi")]
impl MetadataSaver for OpenApi {
    fn save_as(&self, name: &str) -> MetadataSaverResult {
        let openapi_json = self.to_pretty_json()?;

        let path = format!("./assets/openapi/{name}.json");
        fs::write(&path, &openapi_json)?;

        Ok(())
    }
}
