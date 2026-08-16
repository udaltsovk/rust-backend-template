use std::sync::LazyLock;

use is_iso3166_alpha2_country_code::IsIso3166Alpha2CountryCode;
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

#[derive(DomainType, Debug)]
pub struct UserTargetSettingsCountry(String);

static CONSTRAINTS: LazyLock<Constraints<String>> = LazyLock::new(|| {
    Constraints::builder()
        .add_constraint(
            IsIso3166Alpha2CountryCode::with_err(|_| {
                "must be a valid ISO 3166-1 alpha-2 country code".to_string()
            })
            .build(),
        )
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
);

mod is_iso3166_alpha2_country_code {
    use lib::{
        constraint, constraint_check,
        domain::validation::constraints::{Constraint, Validation},
    };

    #[constraint]
    pub struct IsIso3166Alpha2CountryCode {
        err_fn: fn(&str) -> String,
    }

    #[constraint_check(String)]
    impl IsIso3166Alpha2CountryCode {
        fn is_valid(value: &str) -> bool {
            rust_iso3166::from_alpha2(&value.to_uppercase()).is_some()
        }
    }
}
