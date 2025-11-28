use std::collections::HashMap;

use axum::{
    RequestPartsExt as _,
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use serde::Deserialize;

use crate::context::errors::AppError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiVersion {
    V0,
}

impl<S> FromRequestParts<S> for ApiVersion
where
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> = parts.extract().await?;

        let version = params.get("api_version").ok_or_else(|| {
            AppError::UnknownApiVerRejection(
                "missing version param".to_string(),
            )
        })?;

        match version.as_str() {
            "v0" => Ok(Self::V0),
            _ => Err(AppError::UnknownApiVerRejection(version.clone())),
        }
    }
}
