use std::{fmt, sync::LazyLock};

use lib::{
    DomainType,
    domain::validation::{Constraints, error::ValidationErrors},
};

use crate::constraints::EMAIL_CONSTRAINTS;

#[derive(DomainType, Debug)]
pub struct UserEmail(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("email", &EMAIL_CONSTRAINTS).build()
});

impl TryFrom<String> for UserEmail {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS
            .check(&value)
            .into_result(|_| Self(value.to_lowercase()))
    }
}

impl fmt::Display for UserEmail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
