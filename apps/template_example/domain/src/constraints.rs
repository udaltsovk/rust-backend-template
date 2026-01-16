use std::sync::LazyLock;

use lib::domain::validation::constraints::{self, Constraint, ConstraintVec};
use url::Url;

pub static EMAIL_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(constraints::length::Min(8))
            .add_constraint(constraints::length::Max(120))
            .add_constraint(constraints::IsValidEmail)
    });

pub static PASSWORD_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(constraints::length::Min(8))
            .add_constraint(constraints::length::Max(60))
            .add_constraint(constraints::has::Lowercase)
            .add_constraint(constraints::has::Uppercase)
            .add_constraint(constraints::has::Digit)
            .add_constraint(constraints::has::SpecialChar)
    });

pub static URL_CONSTRAINTS: LazyLock<ConstraintVec<String>> =
    LazyLock::new(|| {
        ConstraintVec::new()
            .add_constraint(constraints::length::Max(350))
            .add_constraint(crate::constraints::IsSuitableUrl)
    });

pub struct IsSuitableUrl;

impl Constraint<String> for IsSuitableUrl {
    fn check(&self, value: &String) -> bool {
        Url::parse(value).is_ok_and(|url| {
            ["http", "https"].contains(&url.scheme()) && url.has_host()
        })
    }

    fn error_msg(&self) -> String {
        "is not a suitable url".into()
    }
}
