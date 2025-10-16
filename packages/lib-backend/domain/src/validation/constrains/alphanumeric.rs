use crate::validation::constrains::Constrain;

pub struct IsAlphanumeric;

impl<T> Constrain<T> for IsAlphanumeric
where
    T: ToString,
{
    fn check(&self, value: &T) -> bool {
        value.to_string().chars().all(|c| c.is_alphanumeric())
    }

    fn error_msg(&self) -> String {
        "must contain only alphanumeric characters".to_string()
    }
}
