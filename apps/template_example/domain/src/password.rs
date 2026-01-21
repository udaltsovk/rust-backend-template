use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        try_from_option,
        validation::{
            Constraints,
            error::{ValidationErrors, ValidationResult},
        },
    },
};

use crate::constraints::PASSWORD_CONSTRAINTS;

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct Password(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("password", &PASSWORD_CONSTRAINTS).build()
});

impl TryFrom<String> for Password {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

try_from_option!(
    domain_type = Password,
    from_ty = String,
    constraints = CONSTRAINTS
);

impl Password {
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[cfg_attr(debug_assertions, derive(Debug))]
pub struct PasswordHash(pub String);
