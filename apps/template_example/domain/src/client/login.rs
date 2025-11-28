use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, constraints, error::ValidationErrors},
};

#[derive(DomainType)]
pub struct ClientLogin(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("login")
        .add_constraint(constraints::IsAsciiAlphanumeric)
        .add_constraint(constraints::length::Min(3))
        .add_constraint(constraints::length::Max(32))
        .build()
});

impl TryFrom<String> for ClientLogin {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
