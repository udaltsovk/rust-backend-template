use lib::{
    domain::validation::error::ValidationErrors,
    presentation::api::rest::validation_error_response_openapi as validation_error_response,
};

validation_error_response!(
    /// поля не прошли валидацию
    name = ValidationFailedResponse,
    error_code = "VALIDATION_FAILED",
    status_code = UNPROCESSABLE_ENTITY,
);

impl From<ValidationErrors> for ValidationFailedResponse {
    fn from(errors: ValidationErrors) -> Self {
        Self::new("Some fields haven't passed validation", errors)
    }
}
