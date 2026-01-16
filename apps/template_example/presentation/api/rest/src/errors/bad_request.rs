use axum::extract::rejection::{JsonRejection, PathRejection};
use lib::{
    domain::validation::error::ValidationErrors,
    presentation::api::rest::error_response, tap::Pipe as _,
};

error_response!(
    /// Ошибка в данных запроса. Например, несоответствие ожидаемому формату или не несоблюдение ограничений (на длину, на допустимые символы, ...).
    name = BadRequestResponse,
    error_code = "invalid_request",
    status_code = BAD_REQUEST,
);

impl From<ValidationErrors> for BadRequestResponse {
    fn from(errors: ValidationErrors) -> Self {
        errors
            .into_inner()
            .iter()
            .map(|(path, validation_error)| {
                format!("{path}: {validation_error}")
            })
            .collect::<Vec<_>>()
            .pipe(Self::new)
    }
}

macro_rules! from_axum_rejections {
    ($($name: ident),*) => {
        $(
            impl From<$name> for BadRequestResponse {
                fn from(rejection: $name) -> Self {
                    Self::new(vec![rejection])
                }
            }
        )*
    };
}

from_axum_rejections![JsonRejection, PathRejection];
