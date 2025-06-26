use std::fmt::Display;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use tracing::log::error;
use utoipa::ToSchema;

use crate::context::errors::AppError;

#[derive(Serialize, ToSchema)]
pub struct JsonErrorStruct {
    error_code: String,
    errors: Vec<String>,
}
impl JsonErrorStruct {
    pub(crate) fn new(
        error_code: impl Display,
        errors: Vec<impl Display>,
    ) -> Self {
        Self {
            error_code: error_code.to_string(),
            errors: errors.into_iter().map(|e| e.to_string()).collect(),
        }
    }

    pub(crate) fn as_response(&self, status_code: StatusCode) -> Response {
        (status_code, Json(self)).into_response()
    }
}
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::Validation(validation_errors) => {
                error!("{:?}", validation_errors);

                let mut messages: Vec<String> = Vec::new();
                let errors = validation_errors.into_inner();
                for (path, validation_error) in errors {
                    messages.push(format!("{path}: {validation_error}"));
                }

                JsonErrorStruct::new("invalid_request", messages)
                    .as_response(StatusCode::BAD_REQUEST)
            },
            AppError::JsonRejection(rejection) => {
                error!("{:?}", rejection);

                let messages = vec![rejection];
                JsonErrorStruct::new("invalid_request", messages)
                    .as_response(StatusCode::BAD_REQUEST)
            },
            AppError::ApiPathRejection(rejection) => {
                error!("{:?}", rejection);

                let messages = vec![rejection];
                JsonErrorStruct::new("missing_api_version", messages)
                    .as_response(StatusCode::BAD_REQUEST)
            },
            AppError::UnknownApiVerRejection(version) => {
                let err = format!("Unknown api version ({}).", version);
                error!("{}", err);

                let messages = vec![err];
                JsonErrorStruct::new(
                    "unknown_api_version".to_string(),
                    messages,
                )
                .as_response(StatusCode::NOT_FOUND)
            },
            AppError::UseCase(err) => {
                let messages = vec![err];
                JsonErrorStruct::new(
                    "internal_server_error".to_string(),
                    messages,
                )
                .as_response(StatusCode::INTERNAL_SERVER_ERROR)
            },
        }
        .into_response()
    }
}
