use std::{fs, io};

use utoipa::openapi::OpenApi;

#[derive(thiserror::Error, Debug)]
pub enum OpenAPISaverError {
    #[error("Failed to serialize OpenAPI: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Failed to save OpenAPI: {0}")]
    Save(#[from] io::Error),
}

pub type OpenAPISaverResult = Result<(), OpenAPISaverError>;

pub trait OpenAPISaver {
    fn save_as(&self, name: &str) -> OpenAPISaverResult;
}

impl OpenAPISaver for OpenApi {
    fn save_as(&self, name: &str) -> OpenAPISaverResult {
        let openapi_json = self.to_pretty_json()?;

        fs::write(format!("./assets/openapi/{name}.json"), &openapi_json)?;

        Ok(())
    }
}
