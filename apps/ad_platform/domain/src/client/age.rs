use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        DomainType as _,
        validation::{Constraints, constraints, error::ValidationErrors},
    },
};

#[derive(DomainType)]
pub struct ClientAge(u16);

static CONSTRAINTS: LazyLock<Constraints<i32>> = LazyLock::new(|| {
    Constraints::builder("age")
        .add_constraint(constraints::range::Min(0))
        .add_constraint(constraints::range::Max(255))
        .build()
});

impl TryFrom<i32> for ClientAge {
    type Error = ValidationErrors;

    fn try_from(value: i32) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| {
            Self(value.try_into().unwrap_or_else(
                Self::it_should_be_safe_to_unwrap(CONSTRAINTS.name()),
            ))
        })
    }
}
