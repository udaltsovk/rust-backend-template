use std::{fmt, sync::LazyLock};

use lib::{
    DomainType,
    domain::{
        try_from_option,
        validation::{Constraints, error::ValidationErrors},
    },
};

use crate::constraints::EMAIL_CONSTRAINTS;

#[derive(DomainType, Debug)]
pub struct Email(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("email", &EMAIL_CONSTRAINTS).build()
});

impl TryFrom<String> for Email {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS
            .check(&value)
            .into_result(|_| Self(value.to_lowercase()))
    }
}

try_from_option!(
    domain_type = Email,
    from_ty = String,
    constraints = CONSTRAINTS
);

impl fmt::Display for Email {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
