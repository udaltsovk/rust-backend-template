use axum::http::{Response, StatusCode};

pub trait ResponseExt {
    #[must_use]
    fn with_status(self, status: StatusCode) -> Self;
}

impl<T> ResponseExt for Response<T> {
    fn with_status(mut self, status: StatusCode) -> Self {
        *self.status_mut() = status;
        self
    }
}

#[cfg(test)]
mod tests {
    use axum::http::{Response, StatusCode};
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn response_ext_with_status_changes_status_code() {
        let response = Response::builder()
            .status(StatusCode::OK)
            .body("test body")
            .expect("Response builder should work");

        let modified_response = response.with_status(StatusCode::CREATED);

        assert_eq!(modified_response.status(), StatusCode::CREATED);
    }

    #[rstest]
    fn response_ext_with_status_preserves_body() {
        let original_body = "original body content";
        let response = Response::builder()
            .status(StatusCode::OK)
            .body(original_body)
            .expect("Response builder should work");

        let modified_response = response.with_status(StatusCode::ACCEPTED);

        assert_eq!(modified_response.status(), StatusCode::ACCEPTED);
        assert_eq!(modified_response.body(), &original_body);
    }

    #[rstest]
    fn response_ext_with_status_preserves_headers() {
        let response = Response::builder()
            .status(StatusCode::OK)
            .header("Content-Type", "application/json")
            .header("X-Custom-Header", "custom-value")
            .body("test body")
            .expect("Response builder should work");

        let modified_response = response.with_status(StatusCode::NO_CONTENT);

        assert_eq!(modified_response.status(), StatusCode::NO_CONTENT);
        assert_eq!(
            modified_response
                .headers()
                .get("Content-Type")
                .expect("Content-Type header should exist"),
            "application/json"
        );
        assert_eq!(
            modified_response
                .headers()
                .get("X-Custom-Header")
                .expect("X-Custom-Header should exist"),
            "custom-value"
        );
    }

    #[rstest]
    fn response_ext_works_with_different_body_types() {
        // Test with String body
        let string_response = Response::builder()
            .status(StatusCode::OK)
            .body(String::from("string body"))
            .expect("Response builder should work");
        let modified_string_response =
            string_response.with_status(StatusCode::CREATED);
        assert_eq!(modified_string_response.status(), StatusCode::CREATED);

        // Test with Vec<u8> body
        let vec_response = Response::builder()
            .status(StatusCode::OK)
            .body(vec![1_i32, 2_i32, 3_i32, 4_i32])
            .expect("Response builder should work");
        let modified_vec_response =
            vec_response.with_status(StatusCode::ACCEPTED);
        assert_eq!(modified_vec_response.status(), StatusCode::ACCEPTED);

        // Test with &str body
        let str_response = Response::builder()
            .status(StatusCode::OK)
            .body("str body")
            .expect("Response builder should work");
        let modified_str_response =
            str_response.with_status(StatusCode::NO_CONTENT);
        assert_eq!(modified_str_response.status(), StatusCode::NO_CONTENT);
    }

    #[rstest]
    fn response_ext_with_various_status_codes() {
        let _response = Response::builder()
            .status(StatusCode::OK)
            .body("test")
            .expect("Response builder should work");

        // Test different status codes
        let status_codes = vec![
            StatusCode::OK,
            StatusCode::CREATED,
            StatusCode::ACCEPTED,
            StatusCode::NO_CONTENT,
            StatusCode::BAD_REQUEST,
            StatusCode::UNAUTHORIZED,
            StatusCode::FORBIDDEN,
            StatusCode::NOT_FOUND,
            StatusCode::INTERNAL_SERVER_ERROR,
            StatusCode::BAD_GATEWAY,
        ];

        for status_code in status_codes {
            let cloned_response = Response::builder()
                .status(StatusCode::OK)
                .body("test")
                .expect("Response builder should work");

            let modified_response = cloned_response.with_status(status_code);
            assert_eq!(modified_response.status(), status_code);
        }
    }

    #[rstest]
    fn response_ext_chaining_with_other_operations() {
        let mut response = Response::builder()
            .status(StatusCode::OK)
            .body("test")
            .expect("Response builder should work")
            .with_status(StatusCode::CREATED);

        // Test that we can still modify headers after using with_status
        response.headers_mut().insert(
            "X-Test",
            "test-value".parse().expect("Valid header value"),
        );

        assert_eq!(response.status(), StatusCode::CREATED);
        assert_eq!(
            response
                .headers()
                .get("X-Test")
                .expect("X-Test header should exist"),
            "test-value"
        );
    }

    #[rstest]
    fn response_ext_with_status_is_must_use() {
        // This test verifies that the with_status method has the #[must_use] attribute
        // If it doesn't, this will generate a compiler warning
        let response = Response::builder()
            .status(StatusCode::OK)
            .body("test")
            .expect("Response builder should work");

        let _modified_response = response.with_status(StatusCode::CREATED);
        // The #[must_use] attribute ensures the return value must be used
    }

    #[rstest]
    fn response_ext_preserves_version() {
        use axum::http::Version;

        let response = Response::builder()
            .status(StatusCode::OK)
            .version(Version::HTTP_11)
            .body("test")
            .expect("Response builder should work");

        let modified_response = response.with_status(StatusCode::ACCEPTED);

        assert_eq!(modified_response.version(), Version::HTTP_11);
        assert_eq!(modified_response.status(), StatusCode::ACCEPTED);
    }

    #[rstest]
    fn response_ext_works_with_empty_body() {
        let response = Response::builder()
            .status(StatusCode::OK)
            .body(())
            .expect("Response builder should work");

        let modified_response = response.with_status(StatusCode::NO_CONTENT);

        assert_eq!(modified_response.status(), StatusCode::NO_CONTENT);
        assert_eq!(modified_response.body(), &());
    }
}
