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

use crate::constraints::URL_CONSTRAINTS;

#[derive(DomainType, Debug)]
pub struct UserAvatarUrl(String);

static CONSTRAINTS: LazyLock<Constraints<String>> =
    LazyLock::new(|| Constraints::builder_with(&URL_CONSTRAINTS).build());

impl TryFrom<String> for UserAvatarUrl {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(domain_type = UserAvatarUrl, input_type = String,);
