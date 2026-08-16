use macros::{constraint, constraint_check};

use super::{Constraint, Validation};

#[constraint]
pub struct IsAscii<T>
where
    T: ToString,
{
    err_fn: fn(&T) -> String,
}

#[constraint_check(T)]
impl<T> IsAscii<T>
where
    T: ToString,
{
    fn is_valid(value: &T) -> bool {
        value.to_string().is_ascii()
    }
}
