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
pub struct UserName(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("name", &NAME_SURNAME_CONSTRAINTS).build()
});

impl TryFrom<String> for UserName {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

try_from_option!(
    domain_type = UserName,
    from_ty = String,
    constraints = CONSTRAINTS
);
