use bon::Builder;
use validator_rs::is_valid_email;

use crate::validation::constraints::Constraint;

#[derive(Builder)]
#[builder(derive(Clone), start_fn = with_err)]
pub struct IsValidEmail {
    #[builder(start_fn)]
    err_fn: fn(&str) -> String,
}

impl Constraint<String> for IsValidEmail {
    fn check(&self, value: &String) -> bool {
        is_valid_email(value)
    }

    fn error_msg(&self, rejected_value: &String) -> String {
        (self.err_fn)(rejected_value)
    }
}
