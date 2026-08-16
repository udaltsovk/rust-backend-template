use macros::{constraint, constraint_check};
use validator_rs::is_valid_email;

use super::{Constraint, Validation};

#[constraint]
pub struct IsValidEmail {
    err_fn: fn(&str) -> String,
}

#[constraint_check(String)]
impl IsValidEmail {
    fn is_valid(value: &str) -> bool {
        is_valid_email(value)
    }
}
