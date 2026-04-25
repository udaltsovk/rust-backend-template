use bon::Builder;

use super::Constraint;

#[derive(Builder)]
#[builder(derive(Clone), start_fn = with_err)]
pub struct IsAsciiAlphanumeric<T>
where
    T: ToString,
{
    #[builder(start_fn)]
    err_fn: fn(&T) -> String,
}

impl<T> Constraint<T> for IsAsciiAlphanumeric<T>
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value
            .to_string()
            .chars()
            .all(|c| c.is_ascii_alphanumeric())
    }

    fn error_msg(&self, rejected_value: &T) -> String {
        (self.err_fn)(rejected_value)
    }
}
