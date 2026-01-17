use axum::{
    extract::{FromRequest, Request},
    response::{IntoResponse, Response},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::errors::AppError;
pub struct Json<T>(pub T);

impl<T, S> FromRequest<S> for Json<T>
where
    T: DeserializeOwned,
    S: Send + Sync,
{
    type Rejection = AppError;

    async fn from_request(
        req: Request,
        state: &S,
    ) -> Result<Self, Self::Rejection> {
        axum::Json::<T>::from_request(req, state)
            .await
            .map(|v| v.0)
            .map(Self)
            .map_err(Self::Rejection::from)
    }
}

impl<T> IntoResponse for Json<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        axum::Json::<T>::into_response(axum::Json(self.0))
    }
}
