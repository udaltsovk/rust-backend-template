use lib::DomainType;

use crate::domain::error::ValidationErrors;

#[derive(DomainType)]
pub struct ClientAge(u16);
impl TryFrom<i32> for ClientAge {
    type Error = ValidationErrors;

    fn try_from(value: i32) -> Result<Self, ValidationErrors> {
        let mut errors = vec![];
        if value < 0 {
            errors.push(("age", "Client age can't be below 0"));
        }
        if value > 255 {
            errors.push(("age", "Client age can't be above 255"));
        }
        errors
            .is_empty()
            .then(|| {
                value.try_into().expect(
                    "We've validated age value, so the range should be safe",
                )
            })
            .ok_or_else(|| errors.into())
    }
}
