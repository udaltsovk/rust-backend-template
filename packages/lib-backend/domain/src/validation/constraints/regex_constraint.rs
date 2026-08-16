use macros::{constraint, constraint_check};
use regex::Regex;

use super::{Constraint, Validation};

#[constraint]
pub struct Matches<T>
where
    T: ToString,
{
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
    ) -> Result<MatchesBuilder<T, matches_builder::SetRegex<S>>, regex::Error>
    {
        Regex::try_from(regex).map(|regex| self.regex(regex))
    }
}

#[constraint_check(T)]
impl<T> Matches<T>
where
    T: ToString,
{
    fn is_valid(&self, value: &T) -> bool {
        self.regex.is_match(&value.to_string())
    }
}
