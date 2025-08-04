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
        let (errors, converted): (Vec<_>, Vec<_>) = self
            .into_iter()
            .map(|v| match v.parse() {
                Ok(ok) => (None, Some(ok)),
                Err(err) => (Some(err), None),
            })
            .unzip();
        errors
            .into_iter()
            .flatten()
            .collect::<ValidationErrors>()
            .into_result(|_| converted.into_iter().flatten().collect())
    }
}
