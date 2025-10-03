use lib::{
    DomainType,
    domain::{DomainType as _, validation::error::ValidationErrors},
};
use tap::Tap as _;

#[derive(DomainType)]
pub struct ClientAge(u16);

impl TryFrom<i32> for ClientAge {
    type Error = ValidationErrors;

    fn try_from(value: i32) -> Result<Self, ValidationErrors> {
        const FIELD: &str = "age";

        ValidationErrors::default()
            .tap_mut(|errors| {
                if value < 0 {
                    errors.push(
                        FIELD,
                        format!("Client {FIELD} can't be below 0"),
                    );
                }

                if value > 255 {
                    errors.push(
                        FIELD,
                        format!("Client {FIELD} can't be above 255"),
                    );
                }
            })
            .into_result(|_| {
                Self(
                    value.try_into().unwrap_or_else(
                        Self::it_should_be_safe_to_unwrap(FIELD),
                    ),
                )
            })
    }
}
