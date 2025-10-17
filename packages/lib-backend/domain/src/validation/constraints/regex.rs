use regex::Regex;

use crate::validation::constraints::Constraint;

pub struct Matches(pub Regex);

impl TryFrom<&str> for Matches {
    type Error = regex::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Regex::new(value).map(Self)
    }
}

impl<T> Constraint<T> for Matches
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        self.0.is_match(&value.to_string())
    }

    fn error_msg(&self) -> String {
        format!("must match pattern `{}`", self.0)
    }
}
