use lib::{DomainType, domain::validation::error::ValidationErrors};

#[derive(DomainType)]
pub struct ClientLogin(String);
impl TryFrom<String> for ClientLogin {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        const FIELD: &str = "login";
        let mut errors = ValidationErrors::default();

        if value.chars().any(|char| !char.is_ascii_alphanumeric()) {
            errors.push(
                FIELD,
                "Login can contain only ascii alphanumeric characters",
            );
        }
        if value.chars().count() < 3 {
            errors
                .push(FIELD, "Login length must be at least 3 characters long");
        }
        if value.chars().count() > 32 {
            errors
                .push(FIELD, "Login length must be at most 32 characters long");
        }

        errors.into_result(|_| Self(value))
    }
}
