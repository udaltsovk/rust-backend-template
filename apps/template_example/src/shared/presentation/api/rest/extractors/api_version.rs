use std::collections::HashMap;

use axum::{
    RequestPartsExt as _,
    extract::{FromRequestParts, Path},
    http::request::Parts,
};
use serde::Deserialize;

use crate::shared::presentation::api::rest::ApiError;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ApiVersion {
    V0,
}

impl<S> FromRequestParts<S> for ApiVersion
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let params: Path<HashMap<String, String>> =
            parts.extract().await?;

        let version =
            params.get("api_version").ok_or_else(|| {
                ApiError::UnknownApiVerRejection(
                    "missing version param".to_string(),
                )
            })?;

        match version.as_str() {
            "v0" => Ok(Self::V0),
            _ => Err(ApiError::UnknownApiVerRejection(
                version.clone(),
            )),
        }
    }
}
