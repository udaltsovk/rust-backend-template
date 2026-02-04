use axum::http::StatusCode;
use serde_json::Value;

use crate::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
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
