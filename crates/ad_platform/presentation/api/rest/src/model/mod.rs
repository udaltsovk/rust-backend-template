#![allow(clippy::empty_docs)]

use kernel::domain::error::ValidationErrors;
pub mod client;
pub mod time;

pub(crate) trait ParseableJson<T> {
    fn parse(self) -> Result<T, ValidationErrors>;
}

impl<J, T> ParseableJson<Vec<T>> for Vec<J>
where
    J: ParseableJson<T>,
{
    fn parse(self) -> Result<Vec<T>, ValidationErrors> {
        let mut errors = vec![];
        let converted: Vec<_> = self
            .into_iter()
            .enumerate()
            .map(|(i, v)| {
                v.parse().inspect_err(|err: &ValidationErrors| {
                    let err: ValidationErrors = err
                        .clone()
                        .into_inner()
                        .into_iter()
                        .map(|(path, error)| (format!("{i}.{path}"), error))
                        .collect::<Vec<_>>()
                        .into();
                    errors.push(err)
                })
            })
            .collect();
        errors
            .is_empty()
            .then(|| {
                converted
                    .into_iter()
                    .map(|c| c.expect("error list is empty so it's safe"))
                    .collect()
            })
            .ok_or_else(|| errors.into())
    }
}
