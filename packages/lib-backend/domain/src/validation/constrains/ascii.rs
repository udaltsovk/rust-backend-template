use crate::validation::constrains::Constrain;

pub struct IsAscii;

impl<T> Constrain<T> for IsAscii
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
