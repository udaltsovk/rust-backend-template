use std::{fmt::Debug, sync::Arc};

use derive_where::derive_where;
pub use regex;
use serde::Serialize;

use crate::validation::error::ValidationErrors;

mod alphanumeric;
mod ascii;
mod ascii_alphanumeric;
mod email;
pub mod has;
pub mod length;
pub mod range;
mod regex_constraint;
pub use alphanumeric::IsAlphanumeric;
pub use ascii::IsAscii;
pub use ascii_alphanumeric::IsAsciiAlphanumeric;
pub use email::IsValidEmail;
pub use regex_constraint::Matches;

pub trait Constraint<T> {
    fn check(&self, value: &T) -> bool;

    fn error_msg(&self, rejected_value: &T) -> String;
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

type TypeMismatchFn = fn(&'static str) -> String;

static DEFAULT_TYPE_MISMATCH_FN: TypeMismatchFn =
    |expected| format!("must be {expected}");

pub struct ConstraintsBuilder<T> {
    constraints: ConstraintVec<T>,
    type_mismatch_fn: Option<TypeMismatchFn>,
    none_msg: Option<&'static str>,
    missing_msg: Option<&'static str>,
}

impl<T> ConstraintsBuilder<T> {
    #[must_use]
    pub const fn new() -> Self {
        Self::new_with_constraints(ConstraintVec::new())
    }

    #[must_use]
    pub const fn new_with_constraints(constraints: ConstraintVec<T>) -> Self {
        Self {
            constraints,
            type_mismatch_fn: None,
            none_msg: None,
            missing_msg: None,
        }
    }

    #[must_use]
    pub fn with_type_mismatch_fn(
        mut self,
        msg_fn: fn(&'static str) -> String,
    ) -> Self {
        self.type_mismatch_fn = Some(msg_fn);
        self
    }

    #[must_use]
    pub const fn with_none_msg(mut self, message: &'static str) -> Self {
        self.none_msg = Some(message);
        self
    }

    #[must_use]
    pub const fn with_missing_msg(mut self, message: &'static str) -> Self {
        self.missing_msg = Some(message);
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
            inner: self.constraints,
            type_mismatch_fn: self
                .type_mismatch_fn
                .unwrap_or(DEFAULT_TYPE_MISMATCH_FN),
            none_msg: self.none_msg.unwrap_or("must not be null"),
            missing_msg: self.missing_msg.unwrap_or("must be present"),
        }
    }
}

impl<T> Default for ConstraintsBuilder<T> {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Constraints<T> {
    inner: ConstraintVec<T>,
    type_mismatch_fn: TypeMismatchFn,
    none_msg: &'static str,
    missing_msg: &'static str,
}

impl<T> Constraints<T>
where
    T: Serialize + Debug + Clone + Send + Sync + 'static,
{
    #[must_use]
    pub const fn builder() -> ConstraintsBuilder<T> {
        ConstraintsBuilder::new()
    }

    #[must_use]
    pub fn builder_with(
        constraints: &ConstraintVec<T>,
    ) -> ConstraintsBuilder<T> {
        ConstraintsBuilder::new_with_constraints(constraints.clone())
    }

    #[must_use]
    pub fn derived(source: &Self) -> Self {
        Self {
            inner: source.inner.clone(),
            type_mismatch_fn: source.type_mismatch_fn,
            none_msg: source.none_msg,
            missing_msg: source.missing_msg,
        }
    }

    pub fn check(&self, value: &T) -> ValidationErrors {
        let mut errors = ValidationErrors::new();

        for constraint in &self.inner.0 {
            if !constraint.check(value) {
                let message = constraint.error_msg(value);
                errors.push(message, value.clone());
            }
        }

        errors
    }
}
