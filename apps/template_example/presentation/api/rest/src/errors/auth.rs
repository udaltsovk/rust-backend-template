use axum::http::StatusCode;
use lib::presentation::api::rest::context::InternalErrorStringExt as _;

use crate::ApiError;

#[derive(thiserror::Error, Debug)]
pub enum AuthError {
    #[error("Invalid token")]
    InvalidToken,
}

impl ApiError {
    pub fn invalid_credentials<T>(
        error: T,
    ) -> (StatusCode, &'static str, String)
    where
        T: ToString,
    {
        (
            StatusCode::UNAUTHORIZED,
            "invalid_credentials",
            error.to_internal_error_string("Invalid credentials"),
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
            error,
        }
    }
}
