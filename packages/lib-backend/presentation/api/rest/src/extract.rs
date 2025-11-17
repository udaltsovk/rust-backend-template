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
        #[derive($from)]
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
