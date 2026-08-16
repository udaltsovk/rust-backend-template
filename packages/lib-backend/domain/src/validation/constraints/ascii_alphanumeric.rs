use macros::{constraint, constraint_check};

use super::{Constraint, Validation};

#[constraint]
pub struct IsAsciiAlphanumeric<T>
where
    T: ToString,
{
    err_fn: fn(&T) -> String,
}

#[constraint_check(T)]
impl<T> IsAsciiAlphanumeric<T>
where
    T: ToString,
{
    fn is_valid(value: &T) -> bool {
        value.to_string().chars().all(|c| c.is_ascii_alphanumeric())
    }
}
