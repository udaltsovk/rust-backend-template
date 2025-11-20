use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib::presentation::api::rest::context::JsonErrorStruct;
use tracing::{log::error, warn};

use crate::context::errors::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            Self::Validation(_)
            | Self::JsonRejection(_)
            | Self::ApiPathRejection(_)
            | Self::UnknownApiVerRejection(_) => warn!("{self:?}"),
            Self::UseCase {
                status_code, ..
            } => match status_code {
                c if c.is_server_error() => error!("{self:?}"),
                c if c.is_client_error() => warn!("{self:?}"),
                _ => (),
            },
        }
        match self {
            Self::Validation(validation_errors) => {
                let errors = validation_errors
                    .into_inner()
                    .iter()
                    .map(|(path, validation_error)| {
                        format!("{path}: {validation_error}")
                    })
                    .collect();

                JsonErrorStruct::new(
                    StatusCode::BAD_REQUEST,
                    "invalid_request",
                    errors,
                )
            },
            Self::JsonRejection(rejection) => JsonErrorStruct::new(
                StatusCode::BAD_REQUEST,
                "invalid_request",
                vec![rejection],
            ),
            Self::ApiPathRejection(rejection) => JsonErrorStruct::new(
                StatusCode::BAD_REQUEST,
                "missing_api_version",
                vec![rejection],
            ),
            Self::UnknownApiVerRejection(version) => JsonErrorStruct::new(
                StatusCode::NOT_FOUND,
                "unknown_api_version",
                vec![format!("Unknown api version ({version}).")],
            ),
            Self::UseCase {
                status_code,
                error_code,
                error,
            } => JsonErrorStruct::new(status_code, error_code, vec![error]),
        }
        .into_response()
    }
}
