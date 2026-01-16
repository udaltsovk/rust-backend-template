use std::fmt::Display;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use utoipa::ToSchema;

/// Error response structure
#[derive(Serialize, ToSchema, Debug)]
pub struct JsonErrorStruct {
    /// Response status code
    #[serde(skip)]
    pub(crate) status_code: StatusCode,

    /// Response error code
    pub(crate) error_code: String,

    /// Response error list
    pub(crate) errors: Vec<String>,
}

impl JsonErrorStruct {
    pub fn new<S, E, D>(status_code: S, error_code: E, errors: Vec<D>) -> Self
    where
        S: Into<StatusCode>,
        E: Display,
        D: Display,
    {
        Self {
            status_code: status_code.into(),
            error_code: error_code.to_string(),
            errors: errors.into_iter().map(|e| e.to_string()).collect(),
        }
    }
}

impl IntoResponse for JsonErrorStruct {
    fn into_response(self) -> Response {
        (self.status_code, Json(self)).into_response()
    }
}

pub trait InternalErrorStringExt: ToString + Sized {
    fn to_internal_error_string(self, public: &'static str) -> String {
        self.to_internal_error_string_with_debug(cfg!(debug_assertions), public)
    }

    fn to_internal_error_string_with_debug(
        self,
        is_debug: bool,
        public: &'static str,
    ) -> String {
        if is_debug {
            self.to_string()
        } else {
            public.to_string()
        }
    }
}

impl<T> InternalErrorStringExt for T where T: ToString + Sized {}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[case(StatusCode::BAD_REQUEST, "validation_error", vec!["Field is required", "Invalid format"], StatusCode::BAD_REQUEST, "validation_error", vec!["Field is required".to_string(), "Invalid format".to_string()])]
    #[case(StatusCode::BAD_REQUEST, "client_error", vec!["Bad request"], StatusCode::BAD_REQUEST, "client_error", vec!["Bad request".to_string()])]
    #[case(StatusCode::INTERNAL_SERVER_ERROR, 42_i32, vec![String::from("string error"), "404".to_string()], StatusCode::INTERNAL_SERVER_ERROR, "42", vec!["string error".to_string(), "404".to_string()])]
    #[case(StatusCode::NOT_FOUND, "not_found", Vec::<String>::new(), StatusCode::NOT_FOUND, "not_found", Vec::<String>::new())]
    fn json_error_struct_new<S, E, D>(
        #[case] status_code: S,
        #[case] error_code: E,
        #[case] errors: Vec<D>,
        #[case] expected_status: StatusCode,
        #[case] expected_error_code: &str,
        #[case] expected_errors: Vec<String>,
    ) where
        S: Into<StatusCode>,
        E: Display,
        D: Display,
    {
        let error = JsonErrorStruct::new(status_code, error_code, errors);

        assert_eq!(error.status_code, expected_status);
        assert_eq!(error.error_code, expected_error_code);
        assert_eq!(error.errors, expected_errors);
    }

    #[rstest]
    fn json_error_struct_into_response() {
        let error = JsonErrorStruct::new(
            StatusCode::UNPROCESSABLE_ENTITY,
            "validation_failed",
            vec!["Invalid input"],
        );

        let response = error.into_response();

        assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
        // Note: We can't easily test the JSON body content without more complex setup
        // but the status code verification ensures the IntoResponse trait works correctly
    }

    #[rstest]
    fn json_error_struct_debug_format() {
        let error = JsonErrorStruct::new(
            StatusCode::BAD_REQUEST,
            "test_error",
            vec!["Test message"],
        );

        let debug_str = format!("{error:?}");
        assert!(debug_str.contains("JsonErrorStruct"));
        assert!(debug_str.contains("test_error"));
        assert!(debug_str.contains("Test message"));
    }

    #[derive(Debug)]
    struct TestError {
        message: String,
    }

    impl std::fmt::Display for TestError {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "TestError: {}", self.message)
        }
    }

    #[rstest]
    #[case(
        true,
        "TestError: Something went wrong",
        "TestError: Something went wrong"
    )]
    #[case(
        false,
        "TestError: Something went wrong",
        "Something went wrong on our side..."
    )]
    #[case(true, "Database connection failed", "Database connection failed")]
    #[case(
        false,
        "Database connection failed",
        "Something went wrong on our side..."
    )]
    #[case(true, "Network timeout", "Network timeout")]
    #[case(false, "Network timeout", "Something went wrong on our side...")]
    #[case(true, "500", "500")]
    #[case(false, "500", "Something went wrong on our side...")]
    fn internal_error_string_ext_with_debug(
        #[case] is_debug: bool,
        #[case] input: impl ToString,
        #[case] expected: &str,
    ) {
        let result = input.to_string().to_internal_error_string_with_debug(
            is_debug,
            "Something went wrong on our side...",
        );
        assert_eq!(result, expected);
    }

    #[rstest]
    fn internal_error_string_ext_default_behavior() {
        let error_message = "Default behavior check";
        let internal_error = error_message
            .to_internal_error_string("Something went wrong on our side...");

        if cfg!(debug_assertions) {
            assert_eq!(internal_error, "Default behavior check");
        } else {
            assert_eq!(internal_error, "Something went wrong on our side...");
        }
    }

    #[rstest]
    fn internal_error_string_ext_trait_coverage() {
        let _: String = "test"
            .to_string()
            .to_internal_error_string("Something went wrong on our side...");
        let _: String = "test"
            .to_internal_error_string("Something went wrong on our side...");
        let custom_error = TestError {
            message: "custom".to_string(),
        };
        let _: String = custom_error
            .to_internal_error_string("Something went wrong on our side...");
        let _: String = 42_i32
            .to_internal_error_string("Something went wrong on our side...");
        let _: String = 42.5_f64
            .to_internal_error_string("Something went wrong on our side...");
    }
}
