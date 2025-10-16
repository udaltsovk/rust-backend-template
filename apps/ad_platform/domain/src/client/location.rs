use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constrains, constrains, error::ValidationErrors},
};

#[derive(DomainType)]
pub struct ClientLocation(String);

static CONSTRAINS: LazyLock<Constrains<String>> = LazyLock::new(|| {
    Constrains::builder("location")
        .add_constrain(constrains::length::Min(5))
        .add_constrain(constrains::length::Max(100))
        .build()
});

impl TryFrom<String> for ClientLocation {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINS.check(&value).into_result(|_| Self(value))
    }
}
