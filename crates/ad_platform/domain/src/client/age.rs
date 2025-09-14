use lib::{DomainType, domain::validation::error::ValidationErrors};

#[derive(DomainType)]
pub struct ClientAge(u16);
impl TryFrom<i32> for ClientAge {
    type Error = ValidationErrors;

    fn try_from(value: i32) -> Result<Self, ValidationErrors> {
        const FIELD: &str = "age";
        let mut errors = ValidationErrors::default();

        if value < 0 {
            errors.push(FIELD, "Client age can't be below 0");
        }
        if value > 255 {
            errors.push(FIELD, "Client age can't be above 255");
        }

        errors.into_result(|_| {
            Self(value.try_into().expect(
                "We've validated age value, so the range should be safe",
            ))
        })
    }
}
