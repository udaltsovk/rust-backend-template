use crate::validation::constraints::Constraint;

pub struct IsAscii;

impl<T> Constraint<T> for IsAscii
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value.to_string().is_ascii()
    }

    fn error_msg(&self) -> String {
        "must contain only ascii characters".to_string()
    }
}
