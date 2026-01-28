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
};

use crate::user::constraints::NAME_SURNAME_CONSTRAINTS;

#[derive(DomainType, Debug)]
pub struct UserSurname(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("surname", &NAME_SURNAME_CONSTRAINTS).build()
});

impl TryFrom<String> for UserSurname {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = UserSurname,
    input_type = String,
    constraints = CONSTRAINTS
);
