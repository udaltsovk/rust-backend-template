use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, constraints, error::ValidationErrors},
};

#[derive(DomainType)]
pub struct ClientLocation(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("location")
        .add_constraint(constraints::length::Min(5))
        .add_constraint(constraints::length::Max(100))
        .build()
});

impl TryFrom<String> for ClientLocation {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
