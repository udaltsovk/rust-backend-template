use axum::extract::rejection::{JsonRejection, PathRejection};
use lib::presentation::api::rest::generic_error_response;

generic_error_response!(
    /// невалидный JSON, неподдерживаемый Content-Type
    name = BadRequestResponse,
    error_code = "BAD_REQUEST",
    status_code = BAD_REQUEST,
);

macro_rules! from_axum_rejections {
    ($(($rejection: ident, $name: literal)),*) => {
        $(
            impl From<$rejection> for BadRequestResponse {
                fn from(rejection: $rejection) -> Self {
                    Self::with_context(
                        concat!("Invalid ", $name),
                        serde_json::json!({"error": rejection.to_string()})
                    )
                    .expect("json! output shold be serializible")
                }
            }
        )*
    };
}

from_axum_rejections![(JsonRejection, "JSON"), (PathRejection, "Path")];
