use lib::{DomainType, domain::validation::error::ValidationErrors};
use tap::Tap as _;

#[derive(DomainType)]
pub struct ClientLogin(String);

impl TryFrom<String> for ClientLogin {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        const FIELD: &str = "login";

        ValidationErrors::default()
            .tap_mut(|errors| {
                if value.chars().any(|char| !char.is_ascii_alphanumeric()) {
                    errors.push(
                        FIELD,
                        format!("Client {FIELD} can contain only ascii alphanumeric characters")
                    );
                }
                if value.chars().count() < 3 {
                    errors.push(
                        FIELD,
                        format!("Client {FIELD} length must be at least 3 characters long")
                    );
                }
                if value.chars().count() > 32 {
                    errors.push(
                        FIELD,
                        format!("Client {FIELD} length must be at most 32 characters long")
                    );
                }
            })
            .into_result(|_| Self(value))
    }
}
