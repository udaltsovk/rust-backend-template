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

use crate::user::constraints::NAME_SURNAME_CONSTRAINTS;

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
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

try_from_option!(
    domain_type = UserSurname,
    from_ty = String,
    constraints = CONSTRAINTS
);
