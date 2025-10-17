use crate::validation::constraints::Constraint;

pub struct IsAsciiAlphanumeric;

impl<T> Constraint<T> for IsAsciiAlphanumeric
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value.to_string().chars().all(|c| c.is_ascii_alphanumeric())
    }

    fn error_msg(&self) -> String {
        "must contain only ascii alphanumeric characters".to_string()
    }
}
