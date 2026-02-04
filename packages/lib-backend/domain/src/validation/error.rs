use std::{convert::Infallible, error::Error, fmt};

use serde::Serialize;
use serde_value::Value;

use crate::validation::ValidationConfirmation;

#[derive(Clone, Debug)]
#[must_use]
pub struct ValidationError {
    pub issue: String,
    pub rejected_value: Value,
}

impl fmt::Display for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Issue: {}, Rejected value: {:?}",
            self.issue, self.rejected_value
        )
    }
}

#[derive(Clone, Debug)]
#[must_use]
pub struct ValidationErrors(Vec<ValidationError>);

pub type ValidationResult<T> = Result<T, ValidationErrors>;

impl ValidationErrors {
    pub const fn new() -> Self {
        Self(vec![])
    }

    #[must_use]
    pub fn into_inner(self) -> Vec<ValidationError> {
        self.0
    }

    pub const fn inner_mut(&mut self) -> &mut Vec<ValidationError> {
        &mut self.0
    }

    pub fn inner(&self) -> &[ValidationError] {
        &self.0
    }

    pub fn extend(&mut self, mut other: Self) {
        self.0.append(&mut other.0);
    }

    pub fn with_error<M, V>(issue: M, rejected_value: V) -> Self
    where
        M: ToString,
        V: Serialize,
    {
        let mut this = Self::new();
        this.push(issue, rejected_value);
        this
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "clippy doesn't know that we want &str here too"
    )]
    pub fn push<M, V>(&mut self, issue: M, rejected_value: V)
    where
        M: ToString,
        V: Serialize,
    {
        let error = ValidationError {
            issue: issue.to_string(),
            rejected_value: serde_value::to_value(rejected_value)
                .unwrap_or(Value::Option(None)),
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

        write!(f, "Validation errors: [\n{errors}\n]")
    }
}

impl Error for ValidationErrors {}

impl From<Vec<Self>> for ValidationErrors {
    fn from(errors: Vec<Self>) -> Self {
        errors
            .into_iter()
            .fold(Self::default(), |mut accumulator, error| {
                accumulator.extend(Self(error.0));
                accumulator
            })
    }
}

impl FromIterator<Self> for ValidationErrors {
    fn from_iter<T: IntoIterator<Item = Self>>(iter: T) -> Self {
        iter.into_iter().collect::<Vec<_>>().into()
    }
}

impl From<Infallible> for ValidationErrors {
    fn from(_: Infallible) -> Self {
        Self::new()
    }
}
