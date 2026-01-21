use std::{error::Error, fmt};

use serde::Serialize;
use serde_json::Value;

use crate::validation::ValidationConfirmation;

#[derive(Clone, Debug)]
pub struct ValidationError {
    pub path: String,
    pub issue: String,
    pub rejected_value: Value,
}

impl ValidationError {
    #[must_use]
    pub fn prepend_path<P>(mut self, path: P) -> Self
    where
        P: fmt::Display,
    {
        self.path = format!("{path}.{}", self.path);
        self
    }
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Path: {}, Issue: {}, Rejected value: {}",
            self.path, self.issue, self.rejected_value
        )
    }
}

#[derive(Clone, Debug)]
pub struct ValidationErrors(Vec<ValidationError>);

pub type ValidationResult<T> = Result<T, ValidationErrors>;

impl ValidationErrors {
    #[must_use]
    pub const fn new() -> Self {
        Self(vec![])
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<ValidationError> {
        self.0
    }

    pub fn extend(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

    pub fn with_error<P, M, V>(path: P, issue: M, rejected_value: V) -> Self
    where
        P: ToString,
        M: ToString,
        V: Serialize,
    {
        let mut this = Self::new();
        this.push(path, issue, rejected_value);
        this
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "clippy doesn't know that we want &str here too"
    )]
    pub fn push<P, M, V>(&mut self, path: P, issue: M, rejected_value: V)
    where
        P: ToString,
        M: ToString,
        V: Serialize,
    {
        let error = ValidationError {
            path: path.to_string(),
            issue: issue.to_string(),
            rejected_value: serde_json::to_value(rejected_value)
                .unwrap_or_default(),
        };
        self.0.push(error);
    }

    pub fn into_result<T, F>(self, ok_fn: F) -> Result<T, Self>
    where
        F: FnOnce(ValidationConfirmation) -> T,
    {
        let confirmation = ValidationConfirmation(());
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
            .map(ValidationError::to_string)
            .collect::<Vec<_>>()
            .join("\n");

        format!("Validation errors: [\n{errors}\n]").fmt(f)
    }
}

impl Error for ValidationErrors {}

impl From<Vec<Self>> for ValidationErrors {
    fn from(errors: Vec<Self>) -> Self {
        errors.into_iter().enumerate().fold(
            Self::default(),
            |mut acc, (i, e)| {
                let error_vec =
                    e.0.into_iter()
                        .map(|error| error.prepend_path(i))
                        .collect();
                acc.extend(Self(error_vec));
                acc
            },
        )
    }
}

impl FromIterator<Self> for ValidationErrors {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}
