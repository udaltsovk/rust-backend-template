use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::validation::{
        Constraints,
        error::{ValidationErrors, ValidationResult},
    },
};

use crate::constraints::URL_CONSTRAINTS;

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserAvatarUrl(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder_with("avatar_url", &URL_CONSTRAINTS).build()
});

impl TryFrom<String> for UserAvatarUrl {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}
