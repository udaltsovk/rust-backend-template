use std::{
    fmt::{Debug, Display},
    sync::Arc,
};

use derive_where::derive_where;
use serde::Serialize;

use crate::validation::error::ValidationErrors;

mod alphanumeric;
mod ascii;
mod ascii_alphanumeric;
mod email;
pub mod has;
pub mod length;
pub mod range;
mod regex;

pub use alphanumeric::IsAlphanumeric;
pub use ascii::IsAscii;
pub use ascii_alphanumeric::IsAsciiAlphanumeric;
pub use email::IsValidEmail;
pub use regex::Matches;

pub trait Constraint<T> {
    fn check(&self, value: &T) -> bool;

    fn error_msg(&self) -> String;
}

#[derive_where(Clone)]
pub struct ConstraintVec<T>(Vec<Arc<dyn Constraint<T> + Send + Sync>>);

impl<T> ConstraintVec<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self(Vec::new())
    }

    #[must_use]
    pub fn add_constraint<C>(mut self, constraint: C) -> Self
    where
        C: Constraint<T> + Send + Sync + 'static,
    {
        self.0.push(Arc::new(constraint));
        self
    }
}

impl<T> Default for ConstraintVec<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct ConstraintsBuilder<T> {
    name: &'static str,
    constraints: ConstraintVec<T>,
    none_msg: Option<&'static str>,
}

impl<T> ConstraintsBuilder<T> {
    #[must_use]
    pub const fn new(name: &'static str) -> Self {
        Self::new_with_constraints(name, ConstraintVec::new())
    }

    #[must_use]
    pub const fn new_with_constraints(
        name: &'static str,
        constraints: ConstraintVec<T>,
    ) -> Self {
        Self {
            name,
            constraints,
            none_msg: None,
        }
    }

    #[must_use]
    pub const fn with_none_msg(mut self, message: &'static str) -> Self {
        self.none_msg = Some(message);
        self
    }

    #[must_use]
    pub fn add_constraint<C>(mut self, constraint: C) -> Self
    where
        C: Constraint<T> + Send + Sync + 'static,
    {
        self.constraints = self.constraints.add_constraint(constraint);
        self
    }

    #[must_use]
    pub fn build(self) -> Constraints<T> {
        Constraints {
            name: self.name,
            inner: self.constraints,
            none_msg: self.none_msg.unwrap_or("must be not null"),
        }
    }
}

pub struct Constraints<T> {
    name: &'static str,
    inner: ConstraintVec<T>,
    none_msg: &'static str,
}

impl<T> Constraints<T>
where
    T: Serialize + Debug + Display + Clone + Send + Sync + 'static,
{
    #[must_use]
    pub const fn builder(name: &'static str) -> ConstraintsBuilder<T> {
        ConstraintsBuilder::new(name)
    }

    #[must_use]
    pub fn builder_with(
        name: &'static str,
        constraints: &ConstraintVec<T>,
    ) -> ConstraintsBuilder<T> {
        ConstraintsBuilder::new_with_constraints(name, constraints.clone())
    }

    #[must_use]
    pub const fn name(&self) -> &'static str {
        self.name
    }

    #[must_use]
    pub fn derived(name: &'static str, source: &Self) -> Self {
        Self {
            name,
            inner: source.inner.clone(),
            none_msg: source.none_msg,
        }
    }

    pub fn check(&self, value: &T) -> ValidationErrors {
        let mut errors = ValidationErrors::new();

        for constraint in &self.inner.0 {
            if !constraint.check(value) {
                let message = constraint.error_msg();
                errors.push(self.name, message, value.clone());
            }
        }

        errors
    }

    pub fn none_error(&self) -> ValidationErrors {
        ValidationErrors::with_error(self.name, self.none_msg, None::<T>)
    }
}
