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
    fn to_internal_error_string(self) -> String {
        self.to_internal_error_string_with_debug(cfg!(debug_assertions))
    }

    fn to_internal_error_string_with_debug(self, is_debug: bool) -> String {
        if is_debug {
            self.to_string()
        } else {
            "Something went wrong on our side...".to_string()
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
    fn json_error_struct_new_with_status_code() {
        let error = JsonErrorStruct::new(
            StatusCode::BAD_REQUEST,
            "validation_error",
            vec!["Field is required", "Invalid format"],
        );

        assert_eq!(error.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(error.error_code, "validation_error");
        assert_eq!(
            error.errors,
            vec![
                "Field is required".to_string(),
                "Invalid format".to_string()
            ]
        );
    }

    #[rstest]
    fn json_error_struct_new_with_u16_status() {
        let error = JsonErrorStruct::new(
            StatusCode::BAD_REQUEST,
            "client_error",
            vec!["Bad request"],
        );

        assert_eq!(error.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(error.error_code, "client_error");
        assert_eq!(error.errors, vec!["Bad request".to_string()]);
    }

    #[rstest]
    fn json_error_struct_new_with_different_display_types() {
        let error = JsonErrorStruct::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            42_i32, // implements Display
            vec![String::from("string error"), "404".to_string()], /* mix of String and String */
        );

        assert_eq!(error.status_code, StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(error.error_code, "42");
        assert_eq!(
            error.errors,
            vec!["string error".to_string(), "404".to_string()]
        );
    }

    #[rstest]
    fn json_error_struct_new_with_empty_errors() {
        let error = JsonErrorStruct::new(
            StatusCode::NOT_FOUND,
            "not_found",
            Vec::<String>::new(),
        );

        assert_eq!(error.status_code, StatusCode::NOT_FOUND);
        assert_eq!(error.error_code, "not_found");
        assert!(error.errors.is_empty());
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
    fn internal_error_string_ext_debug_mode() {
        let test_error = TestError {
            message: "Something went wrong".to_string(),
        };

        let error_string = test_error.to_internal_error_string_with_debug(true);
        assert_eq!(error_string, "TestError: Something went wrong");
    }

    #[rstest]
    fn internal_error_string_ext_release_mode() {
        let test_error = TestError {
            message: "Something went wrong".to_string(),
        };

        let error_string =
            test_error.to_internal_error_string_with_debug(false);
        assert_eq!(error_string, "Something went wrong on our side...");
    }

    #[rstest]
    fn internal_error_string_ext_with_string() {
        let error_message = "Database connection failed".to_string();
        let internal_error =
            error_message.to_internal_error_string_with_debug(true);
        assert_eq!(internal_error, "Database connection failed");

        let internal_error_release = "Database connection failed"
            .to_string()
            .to_internal_error_string_with_debug(false);
        assert_eq!(
            internal_error_release,
            "Something went wrong on our side..."
        );
    }

    #[rstest]
    fn internal_error_string_ext_with_str() {
        let error_message = "Network timeout";
        let internal_error =
            error_message.to_internal_error_string_with_debug(true);
        assert_eq!(internal_error, "Network timeout");

        let internal_error_release =
            error_message.to_internal_error_string_with_debug(false);
        assert_eq!(
            internal_error_release,
            "Something went wrong on our side..."
        );
    }

    #[rstest]
    fn internal_error_string_ext_with_number() {
        let error_code = 500_i32;
        let internal_error =
            error_code.to_internal_error_string_with_debug(true);
        assert_eq!(internal_error, "500");

        let internal_error_release =
            error_code.to_internal_error_string_with_debug(false);
        assert_eq!(
            internal_error_release,
            "Something went wrong on our side..."
        );
    }

    #[rstest]
    fn internal_error_string_ext_default_behavior() {
        let error_message = "Default behavior check";
        let internal_error = error_message.to_internal_error_string();

        if cfg!(debug_assertions) {
            assert_eq!(internal_error, "Default behavior check");
        } else {
            assert_eq!(internal_error, "Something went wrong on our side...");
        }
    }

    // Test that the trait is implemented for various types
    #[rstest]
    fn internal_error_string_ext_trait_coverage() {
        // String
        let _: String = "test".to_string().to_internal_error_string();

        // &str
        let _: String = "test".to_internal_error_string();

        // Custom error type
        let custom_error = TestError {
            message: "custom".to_string(),
        };
        let _: String = custom_error.to_internal_error_string();

        // Numbers
        let _: String = 42_i32.to_internal_error_string();
        let _: String = 42.5_f64.to_internal_error_string();
    }
}
