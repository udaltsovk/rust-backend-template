use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};
use lib::presentation::api::rest::context::JsonErrorStruct;
use tracing::log::error;

use crate::context::errors::AppError;

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(validation_errors) => {
                error!("{validation_errors:?}");

                let messages = validation_errors
                    .into_inner()
                    .iter()
                    .map(|(path, validation_error)| {
                        format!("{path}: {validation_error}")
                    })
                    .collect();

                JsonErrorStruct::new("invalid_request", messages)
                    .as_response(StatusCode::BAD_REQUEST)
            },
            AppError::JsonRejection(rejection) => {
                error!("{rejection:?}");

                let messages = vec![rejection];

                JsonErrorStruct::new("invalid_request", messages)
                    .as_response(StatusCode::BAD_REQUEST)
            },
            AppError::ApiPathRejection(rejection) => {
                error!("{rejection:?}");

                let messages = vec![rejection];

                JsonErrorStruct::new("missing_api_version", messages)
                    .as_response(StatusCode::BAD_REQUEST)
            },
            AppError::UnknownApiVerRejection(version) => {
                let err = format!("Unknown api version ({version}).");
                error!("{err}");

                let messages = vec![err];

                JsonErrorStruct::new("unknown_api_version", messages)
                    .as_response(StatusCode::NOT_FOUND)
            },
            AppError::UseCase {
                status_code,
                error_code,
                message,
            } => {
                let messages = vec![message];

                JsonErrorStruct::new(error_code, messages)
                    .as_response(status_code)
            },
        }
        .into_response()
    }
}
