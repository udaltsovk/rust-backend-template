use std::sync::LazyLock;

pub use is_suitable_url::IsSuitableUrl;
use lib::domain::validation::constraints::{self, ConstraintVec};

pub static EMAIL_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(
                constraints::length::Min::with_err(|_, limit| {
                    format!("must be at least {limit} characters long")
                })
                .limit(8)
                .build(),
            )
            .add_constraint(
                constraints::length::Max::with_err(|_, limit| {
                    format!("must be at most {limit} characters long")
                })
                .limit(120)
                .build(),
            )
            .add_constraint(
                constraints::IsValidEmail::with_err(|_| {
                    "must be a valid email".to_string()
                })
                .build(),
            )
    });

pub static PASSWORD_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(
                constraints::length::Min::with_err(|_, limit| {
                    format!("must be at least {limit} characters long")
                })
                .limit(8)
                .build(),
            )
            .add_constraint(
                constraints::length::Max::with_err(|_, limit| {
                    format!("must be at most {limit} characters long")
                })
                .limit(60)
                .build(),
            )
            .add_constraint(
                constraints::has::Lowercase::with_err(|_| {
                    "must contain at least one lowercase letter".to_string()
                })
                .build(),
            )
            .add_constraint(
                constraints::has::Uppercase::with_err(|_| {
                    "must contain at least one uppercase letter".to_string()
                })
                .build(),
            )
            .add_constraint(
                constraints::has::Digit::with_err(|_| {
                    "must contain at least one digit".to_string()
                })
                .build(),
            )
            .add_constraint(
                constraints::has::SpecialChar::with_err(|_| {
                    "must contain at least one special character (@$!%*?&)"
                        .to_string()
                })
                .build(),
            )
    });

pub static URL_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(
                constraints::length::Max::with_err(|_, limit| {
                    format!("must be at most {limit} characters long")
                })
                .limit(350)
                .build(),
            )
            .add_constraint(
                IsSuitableUrl::with_err(|_| {
                    "must be a suitable url".to_string()
                })
                .build(),
            )
    });

mod is_suitable_url {
    use bon::Builder;
    use lib::domain::validation::constraints::Constraint;
    use url::Url;

    #[derive(Builder)]
    #[builder(derive(Clone), start_fn = with_err)]
    pub struct IsSuitableUrl {
        #[builder(start_fn)]
        err_fn: fn(&str) -> String,
    }

    impl Constraint<String> for IsSuitableUrl {
        fn check(&self, value: &String) -> bool {
            Url::parse(value).is_ok_and(|url| {
                ["http", "https"].contains(&url.scheme()) && url.has_host()
            })
        }

        fn error_msg(&self, rejected_value: &String) -> String {
            (self.err_fn)(rejected_value)
        }
    }
}
