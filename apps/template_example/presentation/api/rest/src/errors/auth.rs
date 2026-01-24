use axum::http::StatusCode;
use lib::presentation::api::rest::errors::InternalErrorStringExt as _;
use serde_json::Value;

use crate::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
}

impl ApiError {
    pub fn invalid_credentials<T>(
        error: T,
    ) -> (StatusCode, &'static str, String, Value)
    where
        T: ToString,
    {
        (
            StatusCode::UNAUTHORIZED,
            "invalid_credentials",
            error.to_internal_error_string("Invalid credentials"),
            Value::Null,
        )
    }
}

impl From<AuthError> for ApiError {
    fn from(error: AuthError) -> Self {
        let (status_code, error_code, error) = {
            use AuthError as E;
            use StatusCode as C;
            match error {
                E::InvalidToken => {
                    (C::UNAUTHORIZED, "invalid_token", "Invalid token".into())
                },
            }
        };

        Self::UseCase {
            status_code,
            error_code,
            message: error,
            details: Value::Null,
        }
    }
}
