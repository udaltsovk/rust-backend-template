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
    use bon::Builder;
    use lib::domain::validation::constraints::Constraint;

    #[derive(Builder)]
    #[builder(derive(Clone), start_fn = with_err)]
    pub struct IsIso3166Alpha2CountryCode {
        #[builder(start_fn)]
        err_fn: fn(&str) -> String,
    }

    impl Constraint<String> for IsIso3166Alpha2CountryCode {
        fn check(&self, value: &String) -> bool {
            rust_iso3166::from_alpha2(&value.to_uppercase()).is_some()
        }

        fn error_msg(&self, rejected_value: &String) -> String {
            (self.err_fn)(rejected_value)
        }
    }
}
