use macros::{constraint, constraint_check};

use super::{Constraint, Validation};

#[constraint]
pub struct IsAlphanumeric<T>
where
    T: ToString,
{
    err_fn: fn(&T) -> String,
}

#[constraint_check(T)]
impl<T> IsAlphanumeric<T>
where
    T: ToString,
{
    fn is_valid(value: &T) -> bool {
        value.to_string().chars().all(char::is_alphanumeric)
    }
}
