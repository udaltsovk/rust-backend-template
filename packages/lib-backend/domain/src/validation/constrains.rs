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

pub trait Constrain<T> {
    fn check(&self, value: &T) -> bool;

    fn error_msg(&self) -> String;
}

pub struct ConstrainsBuilder<T> {
    name: &'static str,
    constrains: Vec<Box<dyn Constrain<T> + Send + Sync>>,
}

impl<T> ConstrainsBuilder<T> {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            constrains: Vec::new(),
        }
    }

    pub fn add_constrain(
        mut self,
        constrain: impl Constrain<T> + Send + Sync + 'static,
    ) -> Self {
        self.constrains.push(Box::new(constrain));
        self
    }

    pub fn build(self) -> Constrains<T> {
        Constrains {
            name: self.name,
            constrains: self.constrains,
        }
    }
}

pub struct Constrains<T> {
    name: &'static str,
    constrains: Vec<Box<dyn Constrain<T> + Send + Sync>>,
}

impl<T> Constrains<T> {
    pub fn builder(name: &'static str) -> ConstrainsBuilder<T> {
        ConstrainsBuilder::new(name)
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub fn check(&self, value: &T) -> ValidationErrors {
        ValidationErrors::new().tap_mut(|errors| {
            self.constrains.iter().for_each(|constrain| {
                if constrain.check(value) {
                    return;
                }

                let message = constrain.error_msg();
                errors.push(self.name, message);
            });
        })
    }
}
