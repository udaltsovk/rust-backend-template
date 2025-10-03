use lib::{DomainType, domain::validation::error::ValidationErrors};
use tap::Tap as _;

#[derive(DomainType)]
pub struct ClientLocation(String);

impl TryFrom<String> for ClientLocation {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        const FIELD: &str = "location";

        ValidationErrors::default()
            .tap_mut(|errors| {
                if value.chars().count() < 5 {
                    errors.push(
                        FIELD,
                        format!("Client {FIELD} length must be at least 5 characters long")
                    );
                }
                if value.chars().count() > 100 {
                    errors.push(
                        FIELD,
                        format!("Location {FIELD} must be at most 100 characters long")
                    );
                }
            })
            .into_result(|_| Self(value))
    }
}
