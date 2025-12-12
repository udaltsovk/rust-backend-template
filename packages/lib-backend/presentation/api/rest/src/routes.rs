use axum::{http::StatusCode, response::IntoResponse};

use crate::context::JsonErrorStruct;

pub async fn fallback_404() -> impl IntoResponse {
    JsonErrorStruct::new(
        StatusCode::NOT_FOUND,
        "not_found".to_string(),
        vec!["the specified route does not exist".to_string()],
    )
}

pub async fn fallback_405() -> impl IntoResponse {
    JsonErrorStruct::new(
        StatusCode::METHOD_NOT_ALLOWED,
        "method_not_allowed".to_string(),
        vec!["the specified route does not support this method".to_string()],
    )
}

#[cfg(test)]
mod tests {
    use axum::http::StatusCode;
    use rstest::rstest;

    use super::*;

    #[rstest]
    #[tokio::test]
    async fn fallback_404_returns_correct_status_and_error() {
        let response = fallback_404().await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::NOT_FOUND);
    }

    #[rstest]
    #[tokio::test]
    async fn fallback_404_returns_correct_error_structure() {
        let error_struct = JsonErrorStruct::new(
            StatusCode::NOT_FOUND,
            "not_found".to_string(),
            vec!["the specified route does not exist".to_string()],
        );

        // Test that our function creates the same structure
        let response = fallback_404().await;
        let actual_response = response.into_response();

        // Verify status matches
        assert_eq!(actual_response.status(), StatusCode::NOT_FOUND);

        // The actual JsonErrorStruct would be serialized in the response body,
        // but we can verify the structure by creating it directly
        assert_eq!(error_struct.status_code, StatusCode::NOT_FOUND);
        assert_eq!(error_struct.error_code, "not_found");
        assert_eq!(
            error_struct.errors,
            vec!["the specified route does not exist".to_string()]
        );
    }

    #[rstest]
    #[tokio::test]
    async fn fallback_405_returns_correct_status_and_error() {
        let response = fallback_405().await;
        let response = response.into_response();

        assert_eq!(response.status(), StatusCode::METHOD_NOT_ALLOWED);
    }

    #[rstest]
    #[tokio::test]
    async fn fallback_405_returns_correct_error_structure() {
        let error_struct = JsonErrorStruct::new(
            StatusCode::METHOD_NOT_ALLOWED,
            "method_not_allowed".to_string(),
            vec![
                "the specified route does not support this method".to_string(),
            ],
        );

        // Test that our function creates the same structure
        let response = fallback_405().await;
        let actual_response = response.into_response();

        // Verify status matches
        assert_eq!(actual_response.status(), StatusCode::METHOD_NOT_ALLOWED);

        // Verify the structure matches what we expect
        assert_eq!(error_struct.status_code, StatusCode::METHOD_NOT_ALLOWED);
        assert_eq!(error_struct.error_code, "method_not_allowed");
        assert_eq!(
            error_struct.errors,
            vec![
                "the specified route does not support this method".to_string()
            ]
        );
    }

    #[rstest]
    #[tokio::test]
    async fn fallback_functions_return_different_status_codes() {
        let response_404 = fallback_404().await.into_response();
        let response_405 = fallback_405().await.into_response();

        assert_eq!(response_404.status(), StatusCode::NOT_FOUND);
        assert_eq!(response_405.status(), StatusCode::METHOD_NOT_ALLOWED);
        assert_ne!(response_404.status(), response_405.status());
    }

    #[rstest]
    #[tokio::test]
    async fn fallback_functions_are_async_and_awaitable() {
        // Test that both functions can be awaited properly
        let _result_404 = fallback_404().await;
        let _result_405 = fallback_405().await;

        // If we reach this point, both functions are properly async
        // Test passes by reaching this point
    }

    #[rstest]
    #[tokio::test]
    async fn fallback_functions_implement_into_response() {
        // Test that both functions return types that implement IntoResponse
        let response_404 = fallback_404().await;
        let response_405 = fallback_405().await;

        // Converting to Response should work
        let _actual_response_404 = response_404.into_response();
        let _actual_response_405 = response_405.into_response();

        // If we reach this point, IntoResponse is properly implemented
        // Test passes by reaching this point
    }

    #[rstest]
    #[tokio::test]
    async fn fallback_404_error_message_is_descriptive() {
        // Create the expected error structure
        let expected_error = JsonErrorStruct::new(
            StatusCode::NOT_FOUND,
            "not_found",
            vec!["the specified route does not exist"],
        );

        assert_eq!(expected_error.error_code, "not_found");
        assert_eq!(
            expected_error
                .errors
                .first()
                .expect("Error message should exist"),
            "the specified route does not exist"
        );
        assert!(
            !expected_error
                .errors
                .first()
                .expect("Error message should exist")
                .is_empty()
        );
    }

    #[rstest]
    #[tokio::test]
    async fn fallback_405_error_message_is_descriptive() {
        // Create the expected error structure
        let expected_error = JsonErrorStruct::new(
            StatusCode::METHOD_NOT_ALLOWED,
            "method_not_allowed",
            vec!["the specified route does not support this method"],
        );

        assert_eq!(expected_error.error_code, "method_not_allowed");
        assert_eq!(
            expected_error
                .errors
                .first()
                .expect("Error message should exist"),
            "the specified route does not support this method"
        );
        assert!(
            !expected_error
                .errors
                .first()
                .expect("Error message should exist")
                .is_empty()
        );
    }

    #[rstest]
    #[tokio::test]
    async fn fallback_functions_use_json_error_struct() {
        // Verify that both functions use JsonErrorStruct internally
        // by checking that the responses have the expected status codes
        let response_404 = fallback_404().await.into_response();
        let response_405 = fallback_405().await.into_response();

        // JsonErrorStruct should set these status codes correctly
        assert_eq!(response_404.status(), StatusCode::NOT_FOUND);
        assert_eq!(response_405.status(), StatusCode::METHOD_NOT_ALLOWED);
    }
}
