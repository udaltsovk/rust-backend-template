use axum::{
    extract::{
        FromRequest, FromRequestParts,
        rejection::{JsonRejection, PathRejection, QueryRejection},
    },
    response::{IntoResponse, Response},
};
use serde::Serialize;

macro_rules! extractor {
    ($from:ident, $name: ident, $rejection: ident, $err_code: literal) => {
        #[derive(Debug, Clone, $from)]
        #[from_request(
            via(axum::extract::$name),
            rejection($crate::context::JsonErrorStruct)
        )]
        pub struct $name<T>(pub T);

        impl From<$rejection> for $crate::context::JsonErrorStruct {
            fn from(rejection: $rejection) -> Self {
                Self {
                    status_code: axum::http::StatusCode::BAD_REQUEST,
                    error_code: format!("invalid_{}", $err_code),
                    errors: vec![rejection.body_text()],
                }
            }
        }
    };
}

extractor!(FromRequest, Json, JsonRejection, "json");

impl<T: Serialize> IntoResponse for Json<T> {
    fn into_response(self) -> Response {
        axum::Json(self.0).into_response()
    }
}

extractor!(FromRequestParts, Path, PathRejection, "url");

extractor!(FromRequestParts, Query, QueryRejection, "query");

#[cfg(test)]
mod tests {
    use axum::{
        body::Body,
        extract::{FromRequest as _, FromRequestParts as _, Json as AxumJson},
        http::{Request, StatusCode, header},
        response::IntoResponse as _,
    };
    use rstest::rstest;
    use serde::{Deserialize, Serialize};

    use super::{Json, Path, Query};
    use crate::context::JsonErrorStruct;

    #[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
    struct TestPayload {
        value: String,
    }

    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct QueryPayload {
        count: u32,
    }

    #[rstest]
    fn json_into_response_sets_status_and_content_type() {
        let payload = TestPayload {
            value: "hello-world".to_string(),
        };

        let response = Json(payload).into_response();

        assert_eq!(response.status(), StatusCode::OK);
        let content_type = response
            .headers()
            .get(header::CONTENT_TYPE)
            .expect("content type header must exist");
        assert_eq!(content_type, "application/json");
    }

    #[rstest]
    #[tokio::test]
    async fn json_from_request_extracts_payload() {
        let payload = TestPayload {
            value: "extracted".to_string(),
        };
        let body = serde_json::to_vec(&payload).expect("payload serializes");

        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from(body))
            .expect("request is built");

        let extracted = Json::<TestPayload>::from_request(request, &())
            .await
            .expect("valid json should extract");

        assert_eq!(extracted.0, payload);
    }

    #[rstest]
    #[tokio::test]
    async fn json_rejection_converts_to_json_error_struct() {
        let request = Request::builder()
            .method("POST")
            .uri("/")
            .header(header::CONTENT_TYPE, "application/json")
            .body(Body::from("{\"value\": 42"))
            .expect("request is built");

        let rejection = AxumJson::<TestPayload>::from_request(request, &())
            .await
            .expect_err("invalid json should produce rejection");

        let error: JsonErrorStruct = rejection.into();

        assert_eq!(error.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(error.error_code, "invalid_json");
        assert!(
            error
                .errors
                .first()
                .is_some_and(|message| !message.is_empty()),
            "error message must exist"
        );
    }

    #[rstest]
    #[tokio::test]
    async fn path_rejection_is_converted_to_json_error_struct() {
        let request = Request::builder()
            .uri("/items/not-a-number")
            .body(Body::empty())
            .expect("request is built");
        let (mut parts, _) = request.into_parts();

        let Err(error) = Path::<u32>::from_request_parts(&mut parts, &()).await
        else {
            panic!("missing router metadata should reject");
        };

        assert_eq!(error.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(error.error_code, "invalid_url");
        assert!(
            error
                .errors
                .first()
                .is_some_and(|message| !message.is_empty()),
            "error message must exist"
        );
    }

    #[rstest]
    #[tokio::test]
    async fn query_rejection_is_converted_to_json_error_struct() {
        let request = Request::builder()
            .uri("/search?count=not-a-number")
            .body(Body::empty())
            .expect("request is built");
        let (mut parts, _) = request.into_parts();

        let Err(error) =
            Query::<QueryPayload>::from_request_parts(&mut parts, &()).await
        else {
            panic!("invalid query should reject");
        };

        assert_eq!(error.status_code, StatusCode::BAD_REQUEST);
        assert_eq!(error.error_code, "invalid_query");
        assert!(
            error
                .errors
                .first()
                .is_some_and(|message| !message.is_empty()),
            "error message must exist"
        );
    }

    #[rstest]
    #[tokio::test]
    async fn query_from_request_extracts_payload() {
        let request = Request::builder()
            .uri("/search?count=42")
            .body(Body::empty())
            .expect("request is built");
        let (mut parts, _) = request.into_parts();

        let extracted =
            Query::<QueryPayload>::from_request_parts(&mut parts, &())
                .await
                .expect("valid query should extract");

        assert_eq!(extracted.0.count, 42);
    }

    #[rstest]
    #[tokio::test]
    async fn path_from_request_extracts_payload() {
        use axum::{Router, routing::get};
        use tower::ServiceExt as _;

        let app = Router::new().route(
            "/items/{id}",
            get(|Path(id): Path<u32>| async move { format!("Item {id}") }),
        );

        let request = Request::builder()
            .uri("/items/42")
            .body(Body::empty())
            .expect("request is built");

        let response =
            app.oneshot(request).await.expect("request should succeed");

        assert_eq!(response.status(), StatusCode::OK);
    }

    #[rstest]
    fn path_can_be_instantiated() {
        let path = Path(42_i32);
        assert_eq!(path.0, 42_i32);
    }

    #[rstest]
    fn query_can_be_instantiated() {
        let query = Query(QueryPayload {
            count: 42,
        });
        assert_eq!(query.0.count, 42);
    }

    #[rstest]
    fn json_can_be_instantiated() {
        let payload = TestPayload {
            value: "test".to_string(),
        };
        let json = Json(payload.clone());
        assert_eq!(json.0, payload);
    }

    #[rstest]
    fn extractors_implement_debug() {
        let json = Json(TestPayload {
            value: "test".to_string(),
        });
        let path = Path(42_i32);
        let query = Query(QueryPayload {
            count: 42,
        });

        assert_eq!(
            format!("{json:?}"),
            "Json(TestPayload { value: \"test\" })"
        );
        assert_eq!(format!("{path:?}"), "Path(42)");
        assert_eq!(format!("{query:?}"), "Query(QueryPayload { count: 42 })");
    }

    #[rstest]
    fn extractors_implement_clone() {
        let json = Json(TestPayload {
            value: "test".to_string(),
        });
        let path = Path(42_i32);
        let query = Query(QueryPayload {
            count: 42,
        });

        let json_clone = json.clone();
        let path_clone = path.clone();
        let query_clone = query.clone();

        assert_eq!(json.0, json_clone.0);
        assert_eq!(path.0, path_clone.0);
        assert_eq!(query.0, query_clone.0);
    }
}
