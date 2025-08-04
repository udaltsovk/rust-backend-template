use lib::DomainType;

use crate::domain::error::ValidationErrors;

#[derive(DomainType)]
pub struct ClientLogin(String);
impl TryFrom<String> for ClientLogin {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        let mut errors = vec![];
        if value.chars().any(|char| !char.is_ascii_alphanumeric()) {
            errors.push((
                "login",
                "Logn can contain only ascii alphanumeric characters",
            ));
        }
        if value.chars().count() < 3 {
            errors.push((
                "login",
                "Login length must be at least 3 characters long",
            ));
        }
        if value.chars().count() > 32 {
            errors.push((
                "login",
                "Login length must be at most 32 characters long",
            ));
        }
        errors
            .is_empty()
            .then_some(Self(value))
            .ok_or_else(|| errors.into())
    }
}
