use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        DomainType as _,
        validation::{Constrains, constrains, error::ValidationErrors},
    },
};

#[derive(DomainType)]
pub struct ClientAge(u16);

static CONSTRAINS: LazyLock<Constrains<i32>> = LazyLock::new(|| {
    Constrains::builder("age")
        .add_constrain(constrains::range::Min(0))
        .add_constrain(constrains::range::Max(255))
        .build()
});

impl TryFrom<i32> for ClientAge {
    type Error = ValidationErrors;

    fn try_from(value: i32) -> Result<Self, ValidationErrors> {
        CONSTRAINS.check(&value).into_result(|_| {
            Self(value.try_into().unwrap_or_else(
                Self::it_should_be_safe_to_unwrap(CONSTRAINS.name()),
            ))
        })
    }
}
