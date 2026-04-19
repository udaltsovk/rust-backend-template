use axum::{
    extract::{FromRequest, FromRequestParts},
    response::{IntoResponse, Response},
};
use lib::presentation::api::rest::extractor;
use serde::Serialize;

use super::ApiError;

extractor!(FromRequest, Json, ApiError);

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

extractor!(FromRequestParts, Path, ApiError);

extractor!(FromRequestParts, Query, ApiError);
