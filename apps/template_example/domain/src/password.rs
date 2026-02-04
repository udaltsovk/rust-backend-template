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
    redact::Secret,
    tap::Pipe as _,
};

use crate::constraints::PASSWORD_CONSTRAINTS;

#[derive(DomainType, Debug)]
pub struct Password(Secret<String>);

static CONSTRAINTS: LazyLock<Constraints<String>> =
    LazyLock::new(|| Constraints::builder_with(&PASSWORD_CONSTRAINTS).build());

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
);

#[derive(Debug)]
pub struct PasswordHash(pub Secret<String>);

impl From<String> for PasswordHash {
    fn from(value: String) -> Self {
        value.pipe(Secret::new).pipe(Self)
    }
}
