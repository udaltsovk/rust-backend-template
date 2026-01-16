use validator_rs::is_valid_email;

use crate::validation::constraints::Constraint;

pub struct IsValidEmail;

impl Constraint<String> for IsValidEmail {
    fn check(&self, value: &String) -> bool {
        is_valid_email(value)
    }

    fn error_msg(&self) -> String {
        "must be a valid email".into()
    }
}
