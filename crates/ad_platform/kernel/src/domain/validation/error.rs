use std::{error::Error, fmt, marker::PhantomData};

use crate::domain::validation::ValidationConfirmation;

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

    pub fn into_result<T>(
        self,
        ok_fn: impl FnOnce(ValidationConfirmation) -> T,
    ) -> Result<T, Self> {
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

impl From<Vec<(String, String)>> for ValidationErrors {
    fn from(errors: Vec<(String, String)>) -> Self {
        Self(errors)
    }
}

impl From<Vec<(&str, &str)>> for ValidationErrors {
    fn from(errors: Vec<(&str, &str)>) -> Self {
        Self(
            errors
                .into_iter()
                .map(|(path, err)| (path.to_string(), err.to_string()))
                .collect(),
        )
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
        Self(errors.into_iter().flat_map(|err| err.0).collect())
    }
}
