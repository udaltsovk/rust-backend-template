use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{Constraints, error::ValidationErrors},
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

    fn try_from(value: String) -> Result<Self, ValidationErrors> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
