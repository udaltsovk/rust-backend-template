use lib::presentation::api::rest::{
    errors::validation::FieldErrors,
    fields_error_response_openapi as validation_error_response,
};

validation_error_response!(
    /// поля не прошли валидацию
    name = ValidationFailedResponse,
    error_code = "VALIDATION_FAILED",
    status_code = UNPROCESSABLE_ENTITY,
);

impl From<FieldErrors> for ValidationFailedResponse {
    fn from(errors: FieldErrors) -> Self {
        Self::new("Some fields haven't passed validation", errors)
    }
}
