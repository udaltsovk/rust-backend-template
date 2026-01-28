use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        impl_try_from_external_input,
        validation::{
            Constraints,
            error::{ValidationErrors, ValidationResult},
        },
    },
    tap::Pipe as _,
};
use redact::Secret;

use crate::constraints::PASSWORD_CONSTRAINTS;

#[derive(DomainType, Debug)]
pub struct Password(Secret<String>);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("password", &PASSWORD_CONSTRAINTS).build()
});

impl TryFrom<Secret<String>> for Password {
    type Error = ValidationErrors;

    fn try_from(value: Secret<String>) -> ValidationResult<Self> {
        CONSTRAINTS
            .check(value.expose_secret())
            .into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = Password,
    input_type = Secret<String>,
    constraints = CONSTRAINTS
);

#[derive(Debug)]
pub struct PasswordHash(pub Secret<String>);

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        value.pipe(Secret::new).pipe(Self)
    }
}
