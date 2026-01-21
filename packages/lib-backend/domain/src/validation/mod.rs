pub mod constraints;
pub mod error;
mod option_validator;
mod try_from_option;
mod validator;

pub use constraints::Constraints;
pub use option_validator::{IntoOptionValidator, OptionValidator};
pub use validator::{IntoValidator, Validator};

#[derive(Clone, Copy)]
pub struct ValidationConfirmation(());
