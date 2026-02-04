use application::usecase::user::error::UserUseCaseError;
use axum::http::StatusCode;
use serde_json::{Value, json};

use crate::ApiError;

impl From<UserUseCaseError> for ApiError {
    fn from(error: UserUseCaseError) -> Self {
        let (status_code, error_code, error, details) = {
            use StatusCode as C;
            use UserUseCaseError as E;
            match error {
                E::Infrastructure(_) => Self::internal_server_error(error),

                E::EmailAlreadyUsed(ref email) => (
                    C::CONFLICT,
                    "EMAIL_ALREADY_EXISTS",
                    error.to_string(),
                    json!({
                        "email": email.to_string()
                    }),
                ),

                E::InvalidPassword => (
                    StatusCode::UNAUTHORIZED,
                    "invalid_credentials",
                    "Invalid credentials".into(),
                    Value::Null,
                ),
                E::NotFoundByEmail(ref email) => (
                    C::NOT_FOUND,
                    "NOT_FOUND",
                    error.to_string(),
                    json!({
                        "email": email.to_string()
                    }),
                ),
                E::NotFoundById(id) => (
                    C::NOT_FOUND,
                    "NOT_FOUND",
                    error.to_string(),
                    json!({
                        "user_id": id.to_string()
                    }),
                ),
            }
        };

        Self::UseCase {
            status_code,
            error_code,
            message: error,
            details,
        }
    }
}
