use std::{error::Error, fmt, marker::PhantomData};

use crate::validation::ValidationConfirmation;

#[derive(Clone, Debug)]
pub struct ValidationErrors(Vec<(String, String)>);

impl ValidationErrors {
    pub fn new() -> Self {
        Self(vec![])
    }

    pub fn into_inner(self) -> Vec<(String, String)> {
        self.0
    }

    pub fn extend(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

    pub fn push(&mut self, path: impl ToString, message: impl ToString) {
        self.0.push((path.to_string(), message.to_string()));
    }

    pub fn into_result<T, F>(self, ok_fn: F) -> Result<T, Self>
    where
        F: FnOnce(ValidationConfirmation) -> T,
    {
        let confirmation = ValidationConfirmation {
            _phantom: PhantomData,
        };
        self.0.is_empty().then(|| ok_fn(confirmation)).ok_or(self)
    }
}

impl Default for ValidationErrors {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for ValidationErrors {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let errors = self
            .0
            .iter()
            .map(|(path, err)| format!("{path}: {err}"))
            .collect::<Vec<_>>()
            .join("\n");
        format!("Validation errors: [\n{errors}\n]").fmt(f)
    }
}

impl Error for ValidationErrors {}

impl From<Vec<ValidationErrors>> for ValidationErrors {
    fn from(errors: Vec<ValidationErrors>) -> Self {
        errors.into_iter().enumerate().fold(
            ValidationErrors::default(),
            |mut acc, (i, e)| {
                let errors =
                    e.0.into_iter()
                        .map(|(path, error)| (format!("{i}.{path}"), error))
                        .collect();
                acc.extend(ValidationErrors(errors));
                acc
            },
        )
    }
}

impl FromIterator<ValidationErrors> for ValidationErrors {
    fn from_iter<T: IntoIterator<Item = ValidationErrors>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}
