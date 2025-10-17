use tap::Tap as _;

use crate::validation::error::ValidationErrors;

mod alphanumeric;
mod ascii;
mod ascii_alphanumeric;
pub mod length;
pub mod range;
mod regex;

pub use alphanumeric::IsAlphanumeric;
pub use ascii::IsAscii;
pub use ascii_alphanumeric::IsAsciiAlphanumeric;
pub use regex::Matches;

pub trait Constraint<T> {
    fn check(&self, value: &T) -> bool;

    fn error_msg(&self) -> String;
}

pub struct ConstraintsBuilder<T> {
    name: &'static str,
    constraints: Vec<Box<dyn Constraint<T> + Send + Sync>>,
}

impl<T> ConstraintsBuilder<T> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            constraints: Vec::new(),
        }
    }

    pub fn add_constraint(
        mut self,
        constraint: impl Constraint<T> + Send + Sync + 'static,
    ) -> Self {
        self.constraints.push(Box::new(constraint));
        self
    }

    pub fn build(self) -> Constraints<T> {
        Constraints {
            name: self.name,
            constraints: self.constraints,
        }
    }
}

pub struct Constraints<T> {
    name: &'static str,
    constraints: Vec<Box<dyn Constraint<T> + Send + Sync>>,
}

impl<T> Constraints<T> {
    pub fn builder(name: &'static str) -> ConstraintsBuilder<T> {
        ConstraintsBuilder::new(name)
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn check(&self, value: &T) -> ValidationErrors {
        ValidationErrors::new().tap_mut(|errors| {
            self.constraints.iter().for_each(|constraint| {
                if constraint.check(value) {
                    return;
                }

                let message = constraint.error_msg();
                errors.push(self.name, message);
            });
        })
    }
}
