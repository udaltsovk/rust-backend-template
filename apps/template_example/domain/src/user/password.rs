use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, error::ValidationErrors},
};

use crate::constraints::PASSWORD_CONSTRAINTS;

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserPassword(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("password", &PASSWORD_CONSTRAINTS).build()
});

impl TryFrom<String> for UserPassword {
    type Error = ValidationErrors;

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl UserPassword {
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}
