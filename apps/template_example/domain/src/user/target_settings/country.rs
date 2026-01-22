use std::sync::LazyLock;

use lib::{
    DomainType,
    domain::{
        impl_try_from_external_input,
        validation::{
            Constraints,
            constraints::Constraint,
            error::{ValidationErrors, ValidationResult},
        },
    },
};

#[derive(DomainType)]
#[cfg_attr(debug_assertions, derive(Debug))]
pub struct UserTargetSettingsCountry(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder("country")
        .add_constraint(IsIso3166Alpha2CountryCode)
        .build()
});

impl TryFrom<String> for UserTargetSettingsCountry {
    type Error = ValidationErrors;

    fn try_from(value: String) -> ValidationResult<Self> {
        CONSTRAINTS.check(&value).into_result(|_| Self(value))
    }
}

impl_try_from_external_input!(
    domain_type = UserTargetSettingsCountry,
    input_type = String,
    constraints = CONSTRAINTS
);

struct IsIso3166Alpha2CountryCode;

impl Constraint<String> for IsIso3166Alpha2CountryCode {
    fn check(&self, value: &String) -> bool {
        rust_iso3166::from_alpha2(&value.to_uppercase()).is_some()
    }

    fn error_msg(&self) -> String {
        "is not a valid ISO 3166-1 alpha-2 country code".into()
    }
}
