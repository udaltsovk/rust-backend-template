use lib::{DomainType, domain::validation::error::ValidationErrors};

#[derive(DomainType)]
pub struct ClientLocation(String);
impl TryFrom<String> for ClientLocation {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        const FIELD: &str = "location";
        let mut errors = ValidationErrors::default();

        if value.chars().count() < 5 {
            errors.push(
                FIELD,
                "Location length must be at least 5 characters long",
            );
        }
        if value.chars().count() > 100 {
            errors.push(
                FIELD,
                "Location length must be at most 100 characters long",
            );
        }
        errors.into_result(|_| Self(value))
    }
}
