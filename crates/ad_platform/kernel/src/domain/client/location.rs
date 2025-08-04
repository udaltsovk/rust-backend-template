use lib::DomainType;

use crate::domain::error::ValidationErrors;

#[derive(DomainType)]
pub struct ClientLocation(String);
impl TryFrom<String> for ClientLocation {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        let mut errors = vec![];
        if value.chars().count() < 10 {
            errors.push((
                "location",
                "Location length must be at least 10 characters long",
            ));
        }
        if value.chars().count() > 100 {
            errors.push((
                "location",
                "Location length must be at most 100 characters long",
            ));
        }
        errors
            .is_empty()
            .then_some(Self(value))
            .ok_or_else(|| errors.into())
    }
}
