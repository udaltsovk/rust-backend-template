use bon::Builder;
use regex::Regex;

use super::Constraint;

#[derive(Builder)]
#[builder(derive(Clone), start_fn = with_err)]
pub struct Matches<T>
where
    T: ToString,
{
    #[builder(start_fn)]
    err_fn: fn(&T, &Regex) -> String,
    regex: Regex,
}

impl<T, S> MatchesBuilder<T, S>
where
    T: ToString,
    S: matches_builder::State,
    S::Regex: matches_builder::IsUnset,
{
    pub fn try_regex(
        self,
        regex: &str,
    ) -> Result<
        MatchesBuilder<T, matches_builder::SetRegex<S>>,
        regex::Error,
    > {
        Regex::try_from(regex)
            .map(|regex| self.regex(regex))
    }
}

impl<T> Constraint<T> for Matches<T>
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        self.regex.is_match(&value.to_string())
    }

    fn error_msg(&self, rejected_value: &T) -> String {
        (self.err_fn)(rejected_value, &self.regex)
    }
}
